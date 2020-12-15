use apllodb_immutable_schema_engine_domain::version::{
    id::VersionId, version_number::VersionNumber,
};
use apllodb_shared_components::data_structure::TableName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::sqlite::transaction::sqlite_tx::dao) struct SqliteTableNameForVersion(String);

impl<S: Into<String>> From<S> for SqliteTableNameForVersion {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl SqliteTableNameForVersion {
    pub(in crate::sqlite::transaction::sqlite_tx::dao::version_dao) fn new(
        version_id: &VersionId,
        is_active: bool,
    ) -> Self {
        let s = format!(
            "{}__v{}__{}",
            version_id.vtable_id().table_name(),
            version_id.version_number().to_u64(),
            if is_active { "active" } else { "inactive" }
        );
        Self(s)
    }

    pub(in crate::sqlite::transaction::sqlite_tx::dao::version_dao) fn to_full_table_name(
        &self,
    ) -> TableName {
        TableName::new(self.0.clone()).unwrap()
    }

    pub(in crate::sqlite::transaction::sqlite_tx::dao) fn to_version_number(
        &self,
    ) -> VersionNumber {
        self.split().1
    }

    fn split(&self) -> (TableName, VersionNumber, bool) {
        let parts: Vec<&str> = self.0.split("__").collect();
        assert_eq!(parts.len(), 3);
        (
            TableName::new(parts[0]).unwrap(),
            VersionNumber::from(parts[1][1..].parse::<u64>().unwrap()),
            match parts[2] {
                "active" => true,
                "inactive" => false,
                _ => panic!(
                    "unexpected activation kind from SQLite's table name `{}`",
                    self.0
                ),
            },
        )
    }
}
