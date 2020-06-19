mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

use apllodb_immutable_schema_engine_domain::{ActiveVersion, VTableId};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use create_table_sql_for_version::CreateTableSqlForVersion;
use sqlite_table_name_for_version::SqliteTableNameForVersion;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'tx>,
}

impl<'tx> VersionDao<'tx> {
    pub(in crate::sqlite) fn new(sqlite_tx: &'tx rusqlite::Transaction<'tx>) -> Self {
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
                ApllodbError::new(
                    ApllodbErrorKind::IoError,
                    format!(
                        "SQLite raised an error creating table for version `{:?}`",
                        version.id()
                    ),
                    Some(Box::new(e)),
                )
            })
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn select_active_versions(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<Vec<ActiveVersion>> {
        //         let sql = format!("
        // SELECT
        //         ");

        todo!()
    }
}
