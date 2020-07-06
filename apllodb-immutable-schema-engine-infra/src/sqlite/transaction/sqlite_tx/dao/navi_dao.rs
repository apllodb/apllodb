mod create_table_sql_for_navi;
mod navi;
mod navi_collection;

pub(in crate::sqlite::transaction::sqlite_tx) use navi::{ExistingNavi, Navi};
pub(in crate::sqlite::transaction::sqlite_tx) use navi_collection::NaviCollection;

use crate::sqlite::{sqlite_rowid::SqliteRowid, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    ApparentPrimaryKey, ApparentPrimaryKeyColumnNames, Revision, VTable, VTableId, VersionId,
};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, DataType, DataTypeKind, TableName},
    error::ApllodbResult,
};
use create_table_sql_for_navi::CreateTableSqlForNavi;

#[derive(Debug)]
pub(in crate::sqlite) struct NaviDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx SqliteTx<'db>,
}

pub(in crate::sqlite::transaction::sqlite_tx::dao) const CNAME_ROWID: &str = "rowid";
const TNAME_SUFFIX: &str = "navi";
const CNAME_REVISION: &str = "revision";
const CNAME_VERSION_NUMBER: &str = "version_number";

impl<'tx, 'db: 'tx> NaviDao<'tx, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx::dao) fn table_name(
        vtable_id: &VTableId,
    ) -> ApllodbResult<TableName> {
        let s = format!("{}__{}", vtable_id.table_name(), TNAME_SUFFIX);
        Ok(TableName::new(s)?)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: &'tx SqliteTx<'db>) -> Self {
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
        vtable_id: &VTableId,
        apk_column_names: &ApparentPrimaryKeyColumnNames,
    ) -> ApllodbResult<NaviCollection> {
        use crate::sqlite::to_sql_string::ToSqlString;

        let sql = format!(
            "
SELECT {cname_rowid}, {cname_revision}, {cname_version_number}
  FROM {tname}
  GROUP BY {apk_column_names}
  HAVING
    {cname_revision} = MAX({cname_revision}) AND
    {cname_version_number} IS NOT NULL
",
            cname_rowid = CNAME_ROWID,
            cname_revision = CNAME_REVISION,
            cname_version_number = CNAME_VERSION_NUMBER,
            tname = Self::table_name(vtable_id)?,
            apk_column_names = apk_column_names.to_sql_string(),
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        let cdt_rowid = self.cdt_rowid();
        let cdt_revision = self.cdt_revision();
        let cdt_version_number = self.cdt_version_number();
        let column_data_types = vec![&cdt_rowid, &cdt_revision, &cdt_version_number];

        let row_iter = stmt.query_named(&[], &column_data_types)?;

        Ok(NaviCollection::new(row_iter))
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn probe_latest_revision(
        &self,
        vtable_id: &VTableId,
        apk: &ApparentPrimaryKey,
    ) -> ApllodbResult<Navi> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_storage_engine_interface::PrimaryKey;
        use std::convert::TryFrom;

        let sql = format!(
            "
SELECT {cname_rowid}, {apk_column_names}
  FROM {tname}
  WHERE 
    {apk_condition}
  ORDER BY {cname_revision} DESC
  LIMIT 1;
", // FIXME SQL-i
            cname_rowid = CNAME_ROWID,
            apk_column_names = apk.column_names().to_sql_string(),
            tname = Self::table_name(vtable_id)?,
            apk_condition = apk.to_condition_expression()?.to_sql_string(),
            cname_revision = CNAME_REVISION
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        let cdt_rowid = self.cdt_rowid();
        let mut column_data_types = vec![&cdt_rowid];
        let apk_column_data_types = apk.column_data_types();
        column_data_types.append(
            &mut apk_column_data_types
                .iter()
                .map(|acdt| acdt)
                .collect::<Vec<&ColumnDataType>>(),
        );

        let mut row_iter = stmt.query_named(&[], &column_data_types)?;
        let opt_row = row_iter.next();

        let navi = match opt_row {
            None => Navi::NotExist,
            Some(r) => {
                let r = r?;
                Navi::try_from(r)?
            }
        };
        Ok(navi)
    }

    /// Returns lastly inserted row's ROWID.
    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        apk: ApparentPrimaryKey,
        revision: Revision,
        version_id: &VersionId,
    ) -> ApllodbResult<SqliteRowid> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_storage_engine_interface::PrimaryKey;

        let vtable_id = version_id.vtable_id();

        let sql = format!(
            "INSERT INTO {}__{} ({}, {}, {}) VALUES ({}, :revision, :version_number);",
            vtable_id.table_name(),
            TNAME_SUFFIX,
            apk.column_names()
                .iter()
                .map(|cn| cn.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            CNAME_REVISION,
            CNAME_VERSION_NUMBER,
            apk.sql_values()
                .iter()
                .map(|sql_value| sql_value.to_sql_string())
                .collect::<Vec<String>>()
                .join(", "),
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

    fn cdt_rowid(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_ROWID).unwrap(),
            DataType::new(DataTypeKind::BigInt, false),
        )
    }
    fn cdt_revision(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_REVISION).unwrap(),
            DataType::new(DataTypeKind::BigInt, false),
        )
    }
    fn cdt_version_number(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_VERSION_NUMBER).unwrap(),
            DataType::new(DataTypeKind::BigInt, true),
        )
    }
}
