use apllodb_immutable_schema_engine_domain::ActiveVersion;
use serde::{Deserialize, Serialize};
use crate::sqlite::transaction::sqlite_tx::dao::sqlite_table_name_for_version::SqliteTableNameForVersion;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(super) struct CreateTableSqlForVersion(String);

impl CreateTableSqlForVersion {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&ActiveVersion> for CreateTableSqlForVersion {
    fn from(version: &ActiveVersion) -> Self {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::Entity;

        let version_table_name = SqliteTableNameForVersion::new(version.id(), true);

        let sql = format!(
            "
CREATE TABLE {} (
    _navi_rowid INTEGER NOT NULL,
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

        // TODO materialize Version::constraints

        Self(sql)
    }
}

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx::dao) mod test_wrapper {
    use super::CreateTableSqlForVersion;
    use apllodb_immutable_schema_engine_domain::ActiveVersion;

    /// Provides access to other dao for unit tests.
    pub(in crate::sqlite::transaction::sqlite_tx::dao) struct CreateTableSqlForVersionTestWrapper(
        CreateTableSqlForVersion,
    );
    impl CreateTableSqlForVersionTestWrapper {
        pub(in crate::sqlite::transaction::sqlite_tx::dao) fn from(
            version: &ActiveVersion,
        ) -> Self {
            let inner = CreateTableSqlForVersion::from(version);
            Self(inner)
        }

        pub(in crate::sqlite::transaction::sqlite_tx::dao) fn as_str(&self) -> &str {
            let inner = &self.0;
            &inner.0
        }
    }
}
