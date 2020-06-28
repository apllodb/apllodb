mod active_version_deserializer;

use crate::sqlite::sqlite_error::map_sqlite_err;
use active_version_deserializer::ActiveVersionDeserializer;
use apllodb_immutable_schema_engine_domain::{ActiveVersion, VTableId, VersionId};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};

#[derive(Debug)]
pub(in crate::sqlite) struct SqliteMasterDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'db>,
}

const TNAME: &str = "sqlite_master";
const CNAME_CREATE_TABLE_SQL: &str = "sql";

impl<'tx, 'db: 'tx> SqliteMasterDao<'tx, 'db> {
    pub(in crate::sqlite) fn new(sqlite_tx: &'tx rusqlite::Transaction<'db>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn select_active_versions(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<Vec<ActiveVersion>> {
        let sql = format!(
            r#"
            SELECT {} FROM {} WHERE type = "table" AND name LIKE "{}__v%"
            "#,
            CNAME_CREATE_TABLE_SQL,
            TNAME,
            vtable_id.table_name().as_str()
        );

        let mut stmt = self.sqlite_tx.prepare(&sql).map_err(|e| {
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while preparing for selecting table `{}`",
                    TNAME
                ),
            )
        })?;

        let create_table_sqls: Vec<String> = stmt
            .query_map(rusqlite::NO_PARAMS, |row| row.get(CNAME_CREATE_TABLE_SQL))
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while fetching `{}` column value from table `{}`",
                        CNAME_CREATE_TABLE_SQL, TNAME
                    ),
                )
            })?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!("SQLite raised an error while selecting table `{}`", TNAME),
                )
            })?;

        create_table_sqls
            .iter()
            .map(|create_table_sql| {
                let deserializer = ActiveVersionDeserializer::new(create_table_sql);
                deserializer.into_active_version(vtable_id.database_name())
            })
            .collect::<ApllodbResult<Vec<ActiveVersion>>>()
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn select_active_version(
        &self,
        version_id: &VersionId,
    ) -> ApllodbResult<ActiveVersion> {
        let vtable_id = version_id.vtable_id();
        let mut versions = self.select_active_versions(&vtable_id)?;
        versions.pop().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                format!(
                    "table `{}` not found (or every version is inactive)",
                    vtable_id.table_name()
                ),
                None,
            )
        })
    }
}
