mod create_table_sql_for_navi;
pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) mod navi;
mod navi_table_name;

use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{
    sqlite_rowid::SqliteRowid, sqlite_types::VrrEntries, to_sql_string::ToSqlString,
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    row::pk::{apparent_pk::ApparentPrimaryKey, full_pk::revision::Revision},
    version::id::VersionId,
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::{ApllodbResult, SqlType};
use apllodb_storage_engine_interface::{ColumnDataType, ColumnName};
use create_table_sql_for_navi::CreateTableSqlForNavi;

use self::{
    navi::{ExistingNaviWithPk, Navi},
    navi_table_name::NaviTableName,
};

#[derive(Debug)]
pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) struct NaviDao {
    sqlite_tx: Rc<RefCell<SqliteTx>>,
}

const CNAME_ROWID: &str = "rowid"; // SQLite's keyword
const CNAME_REVISION: &str = "revision";
const CNAME_VERSION_NUMBER: &str = "version_number";

impl NaviDao {
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) fn new(
        sqlite_tx: Rc<RefCell<SqliteTx>>,
    ) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) async fn create_table(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForNavi::from(vtable);
        self.sqlite_tx.borrow_mut().execute(sql.as_str()).await?;
        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) async fn full_scan_latest_revision(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<Vec<ExistingNaviWithPk>> {
        let navi_table_name = NaviTableName::from(vtable.table_name().clone());

        let sql = format!(
            "
SELECT {pk_column_names}, {cname_rowid}, {cname_revision}, {cname_version_number}
  FROM {navi_table_name}
  GROUP BY {pk_column_names}
  HAVING
    {cname_revision} = MAX({cname_revision}) AND
    {cname_version_number} IS NOT NULL
",
            pk_column_names = vtable
                .table_wide_constraints()
                .pk_column_names()
                .to_sql_string(),
            cname_rowid = CNAME_ROWID,
            cname_revision = CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            navi_table_name = navi_table_name.to_sql_string(),
        );

        let cdt_rowid = self.cdt_rowid();
        let cdt_revision = self.cdt_revision();
        let cdt_version_number = self.cdt_version_number();

        let mut column_data_types = vec![&cdt_rowid, &cdt_revision, &cdt_version_number];
        for pk_cdt in vtable.table_wide_constraints().pk_column_data_types() {
            column_data_types.push(pk_cdt);
        }

        let rows = self
            .sqlite_tx
            .borrow_mut()
            .query(
                &sql,
                &navi_table_name.to_table_name(),
                &column_data_types,
                &[],
            )
            .await?;
        let schema = rows.as_schema().clone();

        let ret: Vec<ExistingNaviWithPk> = rows
            .map(|r| ExistingNaviWithPk::from_navi_row(vtable, &schema, r))
            .collect::<ApllodbResult<Vec<Option<ExistingNaviWithPk>>>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(ret)
    }

    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) async fn probe_latest_revision(
        &self,
        vtable_id: &VTableId,
        apk: &ApparentPrimaryKey,
    ) -> ApllodbResult<Navi> {
        let navi_table_name = NaviTableName::from(vtable_id.table_name().clone());

        let sql = format!(
            "
SELECT {cname_rowid}, {cname_version_number}, {cname_revision}
  FROM {navi_table_name} AS {vtable_name}
  WHERE 
    {apk_condition}
  ORDER BY {cname_revision} DESC
  LIMIT 1;
", // FIXME SQL-i
            cname_rowid = CNAME_ROWID,
            cname_revision = CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            navi_table_name = navi_table_name.to_sql_string(),
            vtable_name = vtable_id.table_name().to_sql_string(),
            apk_condition = apk.to_condition_expression()?.to_sql_string(),
        );

        let cdt_rowid = self.cdt_rowid();
        let cdt_revision = self.cdt_revision();
        let cdt_version_number = self.cdt_version_number();
        let column_data_types = vec![&cdt_rowid, &cdt_revision, &cdt_version_number];

        let mut rows = self
            .sqlite_tx
            .borrow_mut()
            .query(
                &sql,
                &navi_table_name.to_table_name(),
                &column_data_types,
                &[],
            )
            .await?;
        let schema = rows.as_schema().clone();

        let navi = match rows.next() {
            None => Navi::NotExist,
            Some(mut r) => Navi::from_navi_row(&schema, &mut r)?,
        };
        Ok(navi)
    }

    /// Returns lastly inserted row's ROWID.
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) async fn insert(
        &self,
        apk: &ApparentPrimaryKey,
        revision: &Revision,
        version_id: &VersionId,
    ) -> ApllodbResult<SqliteRowid> {
        let sql = format!(
            "
            INSERT INTO {navi_table_name} ({pk_column_names}, {cname_revision}, {cname_version_number}) VALUES ({pk_sql_values}, {revision}, {version_number});
            ",  // FIXME SQL-i
            navi_table_name = NaviTableName::from(version_id.vtable_id().table_name().clone()).to_sql_string(),
            pk_column_names = apk.column_names().to_sql_string(),
            cname_revision=CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            pk_sql_values = apk.sql_values().to_sql_string(),
            revision = revision.to_sql_string(),
            version_number = version_id.version_number().to_sql_string(),
        );

        let rowid = self.sqlite_tx.borrow_mut().execute(&sql).await?;

        Ok(rowid)
    }

    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) async fn insert_deleted_records(
        &self,
        vtable: &VTable,
        vrr_entries: VrrEntries,
    ) -> ApllodbResult<()> {
        for vrr_entry in vrr_entries {
            let sql = format!(
                "
                INSERT INTO {navi_table_name} ({pk_column_names}, {cname_revision})
                SELECT {pk_column_names}, {cname_revision} + 1 AS {cname_revision}
                FROM {navi_table_name}
                WHERE
                    {vrr_entry_condition}
                ",
                cname_revision = CNAME_REVISION,
                navi_table_name = NaviTableName::from(vtable.table_name().clone()).to_sql_string(),
                pk_column_names = vtable
                    .table_wide_constraints()
                    .pk_column_names()
                    .to_sql_string(),
                vrr_entry_condition = vrr_entry
                    .to_condition_expression(self.cdt_revision().column_name())?
                    .to_sql_string(),
            );

            let _ = self.sqlite_tx.borrow_mut().execute(&sql).await?;
        }

        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) async fn insert_deleted_records_all(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = format!(
            "
INSERT INTO {navi_table_name} ({pk_column_names}, {cname_revision})
  SELECT {pk_column_names}, {cname_revision} + 1 AS {cname_revision}
    FROM {navi_table_name}
    GROUP BY {pk_column_names}
    HAVING
      {cname_revision} = MAX({cname_revision}) AND
      {cname_version_number} IS NOT NULL
",
            cname_revision = CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            navi_table_name = NaviTableName::from(vtable.table_name().clone()).to_sql_string(),
            pk_column_names = vtable
                .table_wide_constraints()
                .pk_column_names()
                .to_sql_string(),
        );

        let _ = self.sqlite_tx.borrow_mut().execute(&sql).await?;

        Ok(())
    }

    fn cdt_rowid(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_ROWID).unwrap(),
            SqlType::big_int(),
            false,
        )
    }
    fn cdt_revision(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_REVISION).unwrap(),
            SqlType::big_int(),
            false,
        )
    }
    fn cdt_version_number(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_VERSION_NUMBER).unwrap(),
            SqlType::big_int(),
            true,
        )
    }
}
