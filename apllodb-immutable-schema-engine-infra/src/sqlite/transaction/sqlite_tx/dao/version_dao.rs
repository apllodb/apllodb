mod sqlite_table_name_for_version;

use apllodb_immutable_schema_engine_domain::ActiveVersion;
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use sqlite_table_name_for_version::SqliteTableNameForVersion;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'tx>,
}

impl<'tx> VersionDao<'tx> {
    pub(in crate::sqlite) fn new(sqlite_tx: &'tx rusqlite::Transaction<'tx>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite) fn create(&self, version: &ActiveVersion) -> ApllodbResult<()> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::Entity;

        let version_table_name = SqliteTableNameForVersion::new(version.id(), true);

        let sql = format!(
            "
CREATE TABLE {} (
  {}
)
        ",
            version_table_name.as_str(),
            version
                .column_data_types()
                .iter()
                .map(|cdt| cdt.to_sql_string())
                .collect::<Vec<String>>()
                .join(",\n  ")
        );

        self.sqlite_tx
            .execute_named(sql.as_str(), &[])
            .map(|_| ())
            .map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::IoError,
                    format!(
                        "SQLite raised an error creating table `{}`",
                        version_table_name.as_str()
                    ),
                    Some(Box::new(e)),
                )
            })
    }
}
