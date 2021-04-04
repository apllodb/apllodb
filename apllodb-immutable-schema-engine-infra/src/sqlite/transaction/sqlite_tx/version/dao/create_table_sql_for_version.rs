use super::VersionDao;
use apllodb_immutable_schema_engine_domain::version::active_version::ActiveVersion;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(in crate::sqlite::transaction::sqlite_tx) struct CreateTableSqlForVersion(String);

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
