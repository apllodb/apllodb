mod create_table_sql_for_navi;
mod navi;

pub(in crate::sqlite::transaction::sqlite_tx) use navi::{Navi};

use crate::sqlite::{
    sqlite_rowid::SqliteRowid, to_sql_string::ToSqlString, transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row::pk::{apparent_pk::ApparentPrimaryKey, full_pk::revision::Revision},
    version::id::VersionId,
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::{
    data_structure::ColumnDataType,
    data_structure::ColumnName,
    data_structure::ColumnReference,
    data_structure::TableName,
    data_structure::{DataType, DataTypeKind},
    error::ApllodbResult,
};
use create_table_sql_for_navi::CreateTableSqlForNavi;

use self::navi::ExistingNaviWithPK;

#[derive(Debug)]
pub(in crate::sqlite) struct NaviDao<'dao, 'db: 'dao> {
    sqlite_tx: &'dao SqliteTx<'db>,
}

const CNAME_ROWID: &str = "rowid"; // SQLite's keyword
const TNAME_SUFFIX: &str = "navi";
const CNAME_REVISION: &str = "revision";
const CNAME_VERSION_NUMBER: &str = "version_number";

impl<'dao, 'db: 'dao> NaviDao<'dao, 'db> {
    fn table_name(vtable_id: &VTableId) -> String {
        format!("{}__{}", vtable_id.table_name(), TNAME_SUFFIX)
    }

    pub(in crate::sqlite) fn new(sqlite_tx: &'dao SqliteTx<'db>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn create_table(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForNavi::from(vtable);
        self.sqlite_tx.execute_named(sql.as_str(), &vec![])?;
        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn full_scan_latest_revision(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<Vec<ExistingNaviWithPK>> {
        let sql = format!(
            "
SELECT {pk_column_names}, {cname_rowid}, {cname_revision}, {cname_version_number}
  FROM {tname}
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
            tname = Self::table_name(vtable.id()),
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        let cdt_rowid = self.cdt_rowid(vtable.table_name().clone());
        let cdt_revision = self.cdt_revision(vtable.table_name().clone());
        let cdt_version_number = self.cdt_version_number(vtable.table_name().clone());
        let column_data_types = vec![&cdt_rowid, &cdt_revision, &cdt_version_number];

        let row_iter = stmt.query_named(&[], &column_data_types, &[])?;

        let ret: Vec<ExistingNaviWithPK> = row_iter
            .filter_map(|r| ExistingNaviWithPK::from_navi_row(vtable, r).ok().flatten())
            .collect();
        Ok(ret)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn probe_latest_revision(
        &self,
        vtable_id: &VTableId,
        apk: &ApparentPrimaryKey,
    ) -> ApllodbResult<Navi> {
        let sql = format!(
            "
SELECT {cname_rowid}
  FROM {tname}
  WHERE 
    {apk_condition}
  ORDER BY {cname_revision} DESC
  LIMIT 1;
", // FIXME SQL-i
            cname_rowid = CNAME_ROWID,
            tname = Self::table_name(vtable_id),
            apk_condition = apk.to_condition_expression()?.to_sql_string(),
            cname_revision = CNAME_REVISION
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        let cdt_rowid = self.cdt_rowid(vtable_id.table_name().clone());
        let column_data_types = vec![&cdt_rowid];

        let mut row_iter = stmt.query_named(&[], &column_data_types, &[])?;
        let opt_row = row_iter.next();

        let navi = match opt_row {
            None => Navi::NotExist,
            Some(r) => Navi::from_navi_row(vtable_id.table_name(), &r)?,
        };
        Ok(navi)
    }

    /// Returns lastly inserted row's ROWID.
    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        apk: &ApparentPrimaryKey,
        revision: &Revision,
        version_id: &VersionId,
    ) -> ApllodbResult<SqliteRowid> {
        let vtable_id = version_id.vtable_id();

        let sql = format!(
            "INSERT INTO {tname} ({pk_column_names}, {cname_revision}, {cname_version_number}) VALUES ({pk_sql_values}, :revision, :version_number);",
            tname = Self::table_name(vtable_id),
            pk_column_names = apk.column_names().to_sql_string(),
            cname_revision=CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            pk_sql_values = apk.sql_values().to_sql_string(),
        );

        let _ = self.sqlite_tx.execute_named(
            &sql,
            &[
                (":revision", &revision),
                (":version_number", version_id.version_number()),
            ],
        )?;

        Ok(self.sqlite_tx.last_insert_rowid())
    }

    pub(in crate::sqlite) fn insert_deleted_records_all(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = format!(
            "
INSERT INTO {tname} ({pk_column_names}, {cname_revision})
  SELECT {pk_column_names}, {cname_revision} + 1 AS {cname_revision}
    FROM {tname}
    GROUP BY {pk_column_names}
    HAVING
      {cname_revision} = MAX({cname_revision}) AND
      {cname_version_number} IS NOT NULL
",
            cname_revision = CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            tname = Self::table_name(vtable.id()),
            pk_column_names = vtable
                .table_wide_constraints()
                .pk_column_names()
                .to_sql_string(),
        );

        let _ = self.sqlite_tx.execute_named(&sql, &[])?;

        Ok(())
    }

    fn cdt_rowid(&self, table_name: TableName) -> ColumnDataType {
        ColumnDataType::new(
            ColumnReference::new(table_name, ColumnName::new(CNAME_ROWID).unwrap()),
            DataType::new(DataTypeKind::BigInt, false),
        )
    }
    fn cdt_revision(&self, table_name: TableName) -> ColumnDataType {
        ColumnDataType::new(
            ColumnReference::new(table_name, ColumnName::new(CNAME_REVISION).unwrap()),
            DataType::new(DataTypeKind::BigInt, false),
        )
    }
    fn cdt_version_number(&self, table_name: TableName) -> ColumnDataType {
        ColumnDataType::new(
            ColumnReference::new(table_name, ColumnName::new(CNAME_VERSION_NUMBER).unwrap()),
            DataType::new(DataTypeKind::BigInt, true),
        )
    }
}
