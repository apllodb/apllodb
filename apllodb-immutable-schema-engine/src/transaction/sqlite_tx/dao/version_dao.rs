use crate::{transaction::sqlite_tx::sqlite_table_name::SqliteTableNameForVersion, ActiveVersion};
use apllodb_shared_components::{
    data_structure::TableName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};

#[derive(Debug)]
pub(in crate::transaction::sqlite_tx) struct VersionDao<'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'tx>,
    table_name: TableName,
}

impl<'tx> VersionDao<'tx> {
    pub(in crate::transaction::sqlite_tx) fn new(
        sqlite_tx: &'tx rusqlite::Transaction<'tx>,
        table_name: TableName,
    ) -> Self {
        Self {
            sqlite_tx,
            table_name,
        }
    }

    pub(in crate::transaction::sqlite_tx) fn create(
        &self,
        version: &ActiveVersion,
    ) -> ApllodbResult<()> {
        use crate::transaction::sqlite_tx::ToSqlString;

        let version_table_name =
            SqliteTableNameForVersion::new(&self.table_name, version.number(), true);

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
