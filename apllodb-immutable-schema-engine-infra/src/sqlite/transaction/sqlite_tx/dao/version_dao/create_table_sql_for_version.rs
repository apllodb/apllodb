use super::VersionDao;
use apllodb_immutable_schema_engine_domain::version::active_version::ActiveVersion;
use serde::{Deserialize, Serialize};

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
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let version_table_name = VersionDao::table_name(version.id(), true);

        // TODO Make CNAME_NAVI_ROWID primary key for performance.
        let sql = format!(
            "
CREATE TABLE {table_name} (
    {navi_rowid} INTEGER NOT NULL{comma_if_non_pk_columns}
    {non_pk_columns}
)
        ",
            table_name = version_table_name.as_str(),
            navi_rowid = super::CNAME_NAVI_ROWID,
            comma_if_non_pk_columns = if version.column_data_types().is_empty() {
                ""
            } else {
                ","
            },
            non_pk_columns = version.column_data_types().to_sql_string(),
        );

        // TODO materialize Version::constraints

        Self(sql)
    }
}

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx::dao) mod test_wrapper {
    use super::CreateTableSqlForVersion;
    use apllodb_immutable_schema_engine_domain::version::active_version::ActiveVersion;

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
