mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

use crate::sqlite::{sqlite_error::map_sqlite_err, SqliteRowIterator};
use apllodb_immutable_schema_engine_domain::{ActiveVersion, VTableId, VersionId};
use apllodb_shared_components::{
    data_structure::{ColumnName, Expression},
    error::ApllodbResult,
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

    pub(in crate::sqlite::transaction::sqlite_tx) fn full_scan(
        &self,
        version_id: &VersionId,
        column_names: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator<'db>> {
        todo!()
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
            SELECT {} FROM {} WHERE type = "table" AND name LIKE "{}__%"
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
}
