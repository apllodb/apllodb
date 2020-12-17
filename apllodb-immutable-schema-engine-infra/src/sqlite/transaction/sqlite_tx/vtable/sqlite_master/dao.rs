mod active_version_deserializer;

use crate::sqlite::transaction::sqlite_tx::SqliteTx;
use active_version_deserializer::ActiveVersionDeserializer;
use apllodb_immutable_schema_engine_domain::{
    version::{active_version::ActiveVersion, id::VersionId},
    vtable::VTable,
};
use apllodb_shared_components::{
    data_structure::{
        ColumnDataType, ColumnName, ColumnReference, DataType, DataTypeKind, TableName,
    },
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};

#[derive(Debug)]
pub(in crate::sqlite::transaction::sqlite_tx::vtable) struct SqliteMasterDao<'dao, 'db: 'dao> {
    sqlite_tx: &'dao SqliteTx<'db>,
}

const TNAME: &str = "sqlite_master";
const CNAME_CREATE_TABLE_SQL: &str = "sql";

impl<'dao, 'db: 'dao> SqliteMasterDao<'dao, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx::vtable) fn new(
        sqlite_tx: &'dao SqliteTx<'db>,
    ) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx::vtable) fn select_active_versions(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<Vec<ActiveVersion>> {
        use apllodb_storage_engine_interface::Row;

        let sql = format!(
            r#"
            SELECT {} FROM {} WHERE type = "table" AND name LIKE "{}__v%"
            "#,
            CNAME_CREATE_TABLE_SQL,
            TNAME,
            vtable.table_name().as_str()
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;
        let create_table_sqls: Vec<String> = stmt
            .query_named(&[], &[&self.cdt_create_table_sql()], &[])?
            .map(|mut row| {
                let s = row.get::<String>(&ColumnReference::new(
                    TableName::new(TNAME)?,
                    ColumnName::new(CNAME_CREATE_TABLE_SQL)?,
                ))?;
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
    pub(in crate::sqlite::transaction::sqlite_tx::vtable) fn select_active_version(
        &self,
        vtable: &VTable,
        version_id: &VersionId,
    ) -> ApllodbResult<ActiveVersion> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let versions = self.select_active_versions(vtable)?;
        versions
            .into_iter()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!(
                        "table `{}` not found (or every version is inactive)",
                        vtable.table_name()
                    ),
                    None,
                )
            })
    }

    fn cdt_create_table_sql(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnReference::new(
                TableName::new(TNAME).unwrap(),
                ColumnName::new(CNAME_CREATE_TABLE_SQL).unwrap(),
            ),
            DataType::new(DataTypeKind::Text, false),
        )
    }
}