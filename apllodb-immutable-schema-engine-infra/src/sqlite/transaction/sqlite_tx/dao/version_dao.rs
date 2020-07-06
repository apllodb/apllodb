mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

pub(in crate::sqlite::transaction::sqlite_tx::dao) use sqlite_table_name_for_version::SqliteTableNameForVersion;

use super::{navi_dao, NaviDao};
use crate::sqlite::{sqlite_rowid::SqliteRowid, SqliteRowIterator, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    ActiveVersion, ApparentPrimaryKeyColumnNames, VersionId,
};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, Expression, TableName},
    error::ApllodbResult,
};
use create_table_sql_for_version::CreateTableSqlForVersion;
use std::collections::HashMap;

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx::dao) use create_table_sql_for_version::test_wrapper::CreateTableSqlForVersionTestWrapper;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx SqliteTx<'db>,
}

pub(in crate::sqlite::transaction::sqlite_tx::dao) const CNAME_NAVI_ROWID: &str = "_navi_rowid";

impl VersionDao<'_, '_> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn table_name(
        version_id: &VersionId,
        is_active: bool,
    ) -> TableName {
        SqliteTableNameForVersion::new(version_id, is_active).to_table_name()
    }
}

impl<'tx, 'db: 'tx> VersionDao<'tx, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: &'tx SqliteTx<'db>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn create_table(
        &self,
        version: &ActiveVersion,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForVersion::from(version);
        self.sqlite_tx.execute_named(sql.as_str(), &[])?;
        Ok(())
    }

    /// Fetches only existing columns from SQLite, joining ApparentPrimaryKey from navi table.
    pub(in crate::sqlite::transaction::sqlite_tx) fn join_with_navi(
        &self,
        version: &ActiveVersion,
        navi_rowids: &[SqliteRowid],
        apk_column_names: &ApparentPrimaryKeyColumnNames,
        column_names: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::Entity;

        let column_data_types = version.column_data_types();

        // Filter existing and requested columns.
        let column_data_types: Vec<&ColumnDataType> = column_data_types
            .iter()
            .filter(|cdt| column_names.contains(cdt.column_name()))
            .collect();

        let sqlite_table_name = Self::table_name(version.id(), true);

        let sql = format!(
            "
SELECT {}, {} FROM :version_table
  INNER JOIN :navi_table
    ON :version_table.:version_navi_rowid = :navi_table.:navi_rowid
  WHERE :version_table.:version_navi_rowid IN (:navi_rowids)
", // FIXME prevent SQL injection
            apk_column_names.to_sql_string(),
            column_data_types
                .iter()
                .map(|cdt| cdt.column_name().as_str())
                .collect::<Vec<&str>>()
                .join(", "),
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        // TODO SqliteRowIteratorには、PKを含むRoをを作って貰う必要があるので、PKの情報も渡す必要あり
        let row_iter = stmt.query_named(
            &[
                ("version_table", &sqlite_table_name),
                ("navi_table", &NaviDao::table_name(version.vtable_id())?),
                ("version_navi_rowid", &CNAME_NAVI_ROWID),
                ("navi_rowid", &navi_dao::CNAME_ROWID),
                ("navi_rowids", &navi_rowids),
            ],
            &column_data_types,
        )?;
        Ok(row_iter)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        version_id: &VersionId,
        navi_rowid: SqliteRowid,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        use crate::sqlite::to_sql_string::ToSqlString;

        let sqlite_table_name = Self::table_name(version_id, true);
        let sql = format!(
            "
        INSERT INTO {}
          ({}, {})
          VALUES ({}, {})
        ", // FIXME might lead to SQL injection.
            sqlite_table_name.as_str(),
            CNAME_NAVI_ROWID,
            column_values
                .keys()
                .map(|cn| cn.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            navi_rowid.0,
            column_values
                .values()
                .map(|expr| expr.to_sql_string())
                .collect::<Vec<String>>()
                .join(", "),
        );

        self.sqlite_tx.execute_named(&sql, &vec![])?;

        Ok(())
    }
}
