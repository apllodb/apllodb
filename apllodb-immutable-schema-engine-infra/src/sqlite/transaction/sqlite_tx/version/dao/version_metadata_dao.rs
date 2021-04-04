use std::{cell::RefCell, rc::Rc};

use crate::sqlite::transaction::sqlite_tx::SqliteTx;

use apllodb_immutable_schema_engine_domain::{
    version::{active_version::ActiveVersion, id::VersionId},
    vtable::VTable,
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnDataType, ColumnName, SqlType, TableName,
};

#[derive(Debug)]
pub(in crate::sqlite::transaction::sqlite_tx) struct VersionMetadataDao {
    sqlite_tx: Rc<RefCell<SqliteTx>>,
}

const TNAME: &str = "sqlite_master";
const CNAME_CREATE_TABLE_SQL: &str = "sql";

impl VersionMetadataDao {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: Rc<RefCell<SqliteTx>>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) async fn select_active_versions(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<Vec<ActiveVersion>> {
        let sql = format!(
            r#"
            SELECT {} FROM {} WHERE type = "table" AND name LIKE "{}__v%"
            "#,
            CNAME_CREATE_TABLE_SQL,
            TNAME,
            vtable.table_name().as_str()
        );

        let tname = TableName::new(TNAME)?;

        let create_table_sqls: Vec<String> = self
            .sqlite_tx
            .borrow_mut()
            .query(&sql, &tname, &[&self.cdt_create_table_sql()], &[])
            .await?
            .map(|mut row| {
                let s = row
                    .get::<String>(&ColumnName::new(CNAME_CREATE_TABLE_SQL)?)?
                    .expect("must be NOT NULL");
                Ok(s)
            })
            .collect::<ApllodbResult<Vec<String>>>()?;

        create_table_sqls
            .iter()
            .map(|create_table_sql| {
                let deserializer = ActiveVersionDeserializer::new(create_table_sql);
                deserializer.to_active_version(vtable)
            })
            .collect::<ApllodbResult<Vec<ActiveVersion>>>()
    }

    // TODO 消す
    pub(in crate::sqlite::transaction::sqlite_tx::vtable) async fn select_active_version(
        &self,
        vtable: &VTable,
        version_id: &VersionId,
    ) -> ApllodbResult<ActiveVersion> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let versions = self.select_active_versions(vtable).await?;
        versions
            .into_iter()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!(
                        "table `{:?}` not found (or every version is inactive)",
                        vtable.table_name()
                    ),
                    None,
                )
            })
    }

    fn cdt_create_table_sql(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_CREATE_TABLE_SQL).unwrap(),
            SqlType::text(),
            false,
        )
    }
}
