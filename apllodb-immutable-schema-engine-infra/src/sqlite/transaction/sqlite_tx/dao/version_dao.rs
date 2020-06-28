mod create_table_sql_for_version;

use super::sqlite_table_name_for_version::SqliteTableNameForVersion;
use crate::sqlite::{sqlite_rowid::SqliteRowid, SqliteRowIterator, SqliteTx};
use apllodb_immutable_schema_engine_domain::{ActiveVersion, VersionId};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, Expression},
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

impl<'tx, 'db: 'tx> VersionDao<'tx, 'db> {
    pub(in crate::sqlite) fn new(sqlite_tx: &'tx SqliteTx<'db>) -> Self {
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

    /// Fetches only existing columns from SQLite.
    ///
    /// TODO シグネチャが根本的に間違っている。
    /// 特定バージョンから全レコードを取る操作はしない。同一バージョン内に、同一APK同士の古いrevisionも含まれる可能性があり、そのレコードを取得するのは無駄なので。
    pub(in crate::sqlite::transaction::sqlite_tx) fn full_scan(
        &self,
        version: &ActiveVersion,
        column_names: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        use apllodb_immutable_schema_engine_domain::Entity;

        // TODO
        // ここで navi テーブルを参照して、APKごとに最大のRevisionを特定する。
        // 最大のRevisionを書くバージョンに対して取りに行く

        let column_data_types = version.column_data_types();

        // Filter existing and requested columns.
        let column_data_types: Vec<&ColumnDataType> = column_data_types
            .iter()
            .filter(|cdt| column_names.contains(cdt.column_name()))
            .collect();

        let sqlite_table_name = SqliteTableNameForVersion::new(version.id(), true);

        let sql = format!(
            "
SELECT {} FROM {}
  ", // FIXME prevent SQL injection
            column_data_types
                .iter()
                .map(|cdt| cdt.column_name().as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            sqlite_table_name.as_str(),
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        // TODO SqliteRowIteratorには、PKを含むRoをを作って貰う必要があるので、PKの情報も渡す必要あり
        let row_iter = stmt.query_named(&[], &column_data_types)?;
        Ok(row_iter)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        version_id: &VersionId,
        navi_rowid: SqliteRowid,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        use crate::sqlite::to_sql_string::ToSqlString;

        let sqlite_table_name = SqliteTableNameForVersion::new(version_id, true);
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
