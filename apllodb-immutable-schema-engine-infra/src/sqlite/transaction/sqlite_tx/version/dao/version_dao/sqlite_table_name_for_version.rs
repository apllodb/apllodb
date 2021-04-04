use apllodb_immutable_schema_engine_domain::version::id::VersionId;
use apllodb_shared_components::TableName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::sqlite::transaction::sqlite_tx) struct SqliteTableNameForVersion(String);

impl<S: Into<String>> From<S> for SqliteTableNameForVersion {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl SqliteTableNameForVersion {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(version_id: &VersionId) -> Self {
        let s = format!(
            "{}__v{}",
            version_id.vtable_id().table_name().as_str(),
            version_id.version_number().to_u64(),
        );
        Self(s)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn to_full_table_name(&self) -> TableName {
        TableName::new(self.0.clone()).unwrap()
    }
}
