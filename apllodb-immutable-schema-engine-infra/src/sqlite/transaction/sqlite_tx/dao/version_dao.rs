mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

use crate::sqlite::{sqlite_error::map_sqlite_err, SqliteRowIterator};
use apllodb_immutable_schema_engine_domain::{ActiveVersion, VTableId, VersionId};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, Expression},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use create_table_sql_for_version::CreateTableSqlForVersion;
use sqlite_table_name_for_version::SqliteTableNameForVersion;
use std::collections::HashMap;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'db>,
}

const TABLE_NAME_SQLITE_MASTER: &str = "sqlite_master";
const CNAME_SQLITE_MASTER_SQL: &str = "sql";

impl<'tx, 'db: 'tx> VersionDao<'tx, 'db> {
    pub(in crate::sqlite) fn new(sqlite_tx: &'tx rusqlite::Transaction<'db>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn create(
        &self,
        version: &ActiveVersion,
    ) -> ApllodbResult<()> {
        use apllodb_immutable_schema_engine_domain::Entity;

        let sql = CreateTableSqlForVersion::from(version);

        self.sqlite_tx
            .execute_named(sql.as_str(), &[])
            .map(|_| ())
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error creating table for version `{:?}`",
                        version.id()
                    ),
                )
            })
    }

    /// Fetches only existing columns from SQLite.
    ///
    /// TODO シグネチャが根本的に間違っている。
    /// 特定バージョンから全レコードを取る操作はしない。同一バージョン内に、同一APK同士の古いrevisionも含まれる可能性があり、そのレコードを取得するのは無駄なので。
    pub(in crate::sqlite::transaction::sqlite_tx) fn full_scan(
        &self,
        version_id: &VersionId,
        column_names: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        // TODO
        // ここで navi テーブルを参照して、APKごとに最大のRevisionを特定する。
        // 最大のRevisionを書くバージョンに対して取りに行く

        let version: ActiveVersion = self.select_active_version(&version_id)?;

        let column_data_types = version.column_data_types();

        // Filter existing and requested columns.
        let column_data_types: Vec<&ColumnDataType> = column_data_types
            .iter()
            .filter(|cdt| column_names.contains(cdt.column_name()))
            .collect();

        let sqlite_table_name = SqliteTableNameForVersion::new(version_id, true);

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

        let mut stmt: rusqlite::Statement = self.sqlite_tx.prepare(&sql).map_err(|e| {
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while selecting (prepare) rows: {}",
                    sql
                ),
            )
        })?;
        let mut rows: rusqlite::Rows = stmt.query_named(
            &[],
        ).map_err(|e| map_sqlite_err(e, "failed to query_map_named (source error type ToSqlConversionFailure does not have any good meaning)"))?;

        // TODO SqliteRowIteratorには、PKを含むRoをを作って貰う必要があるので、PKの情報も渡す必要あり

        let iter = SqliteRowIterator::new(&mut rows, &column_data_types)?;
        Ok(iter)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        version_id: &VersionId,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        use crate::sqlite::to_sql_string::ToSqlString;

        let sqlite_table_name = SqliteTableNameForVersion::new(version_id, true);
        let sql = format!(
            "
        INSERT INTO {}
          ({})
          VALUES ({})
        ", // FIXME might lead to SQL injection.
            sqlite_table_name.as_str(),
            column_values
                .keys()
                .map(|cn| cn.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            column_values
                .values()
                .map(|expr| expr.to_sql_string())
                .collect::<Vec<String>>()
                .join(", "),
        );

        self.sqlite_tx
            .execute(&sql, rusqlite::NO_PARAMS)
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "failed to insert a record into SQLite with this command: {}",
                        sql
                    ),
                )
            })?;

        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn select_active_versions(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<Vec<ActiveVersion>> {
        let sql = format!(
            r#"
            SELECT {} FROM {} WHERE type = "table" AND name LIKE "{}__v%"
            "#,
            CNAME_SQLITE_MASTER_SQL,
            TABLE_NAME_SQLITE_MASTER,
            vtable_id.table_name().as_str()
        );

        let mut stmt = self.sqlite_tx.prepare(&sql).map_err(|e| {
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while preparing for selecting table `{}`",
                    TABLE_NAME_SQLITE_MASTER
                ),
            )
        })?;

        let create_table_sqls: Vec<String> = stmt
            .query_map(rusqlite::NO_PARAMS, |row| row.get(CNAME_SQLITE_MASTER_SQL))
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while fetching `{}` column value from table `{}`",
                        CNAME_SQLITE_MASTER_SQL, TABLE_NAME_SQLITE_MASTER
                    ),
                )
            })?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while selecting table `{}`",
                        TABLE_NAME_SQLITE_MASTER
                    ),
                )
            })?;

        create_table_sqls
            .iter()
            .map(|create_table_sql| {
                let create_table_sql = CreateTableSqlForVersion::new(create_table_sql);
                create_table_sql.into_active_version(vtable_id.database_name())
            })
            .collect::<ApllodbResult<Vec<ActiveVersion>>>()
    }

    fn select_active_version(&self, version_id: &VersionId) -> ApllodbResult<ActiveVersion> {
        let sqlite_table_name = SqliteTableNameForVersion::new(version_id, true);

        let sql = format!(
            r#"
            SELECT {} FROM {} WHERE type = "table" AND name = "{}"
            "#,
            CNAME_SQLITE_MASTER_SQL,
            TABLE_NAME_SQLITE_MASTER,
            sqlite_table_name.as_str()
        );

        let mut stmt = self.sqlite_tx.prepare(&sql).map_err(|e| {
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while preparing for selecting table `{}`",
                    TABLE_NAME_SQLITE_MASTER
                ),
            )
        })?;

        let rows = stmt
            .query_map(rusqlite::NO_PARAMS, |row| row.get(CNAME_SQLITE_MASTER_SQL))
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while fetching `{}` column value from table `{}`",
                        CNAME_SQLITE_MASTER_SQL, TABLE_NAME_SQLITE_MASTER
                    ),
                )
            })?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while selecting table `{}`",
                        TABLE_NAME_SQLITE_MASTER
                    ),
                )
            })?;
        let create_table_sql: &str = rows.first().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                format!(
                    "SQLite table `{}` does not exist",
                    sqlite_table_name.as_str()
                ),
                None,
            )
        })?;

        let create_table_sql = CreateTableSqlForVersion::new(create_table_sql);
        let active_version =
            create_table_sql.into_active_version(version_id.vtable_id().database_name())?;
        Ok(active_version)
    }
}
