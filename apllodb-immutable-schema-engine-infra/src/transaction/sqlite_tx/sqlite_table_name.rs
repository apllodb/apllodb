use crate::VersionNumber;
use apllodb_shared_components::data_structure::TableName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::transaction::sqlite_tx) struct SqliteTableNameForTable;
impl SqliteTableNameForTable {
    pub(in crate::transaction::sqlite_tx) fn name() -> String {
        "_table_metadata".to_string()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::transaction::sqlite_tx) struct SqliteTableNameForVersion(String);

impl<S: Into<String>> From<S> for SqliteTableNameForVersion {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

#[allow(dead_code)]
impl SqliteTableNameForVersion {
    pub(in crate::transaction::sqlite_tx) fn new(
        table_name: &TableName,
        version_number: &VersionNumber,
        is_active: bool,
    ) -> Self {
        let s = format!(
            "{}__{}__{}",
            table_name,
            version_number.to_u64(),
            if is_active { "active" } else { "inactive" }
        );
        Self(s.into())
    }

    pub(in crate::transaction::sqlite_tx) fn to_table_name(&self) -> TableName {
        self.split().0
    }
    pub(in crate::transaction::sqlite_tx) fn to_version_number(&self) -> VersionNumber {
        self.split().1
    }
    pub(in crate::transaction::sqlite_tx) fn is_active(&self) -> bool {
        self.split().2
    }

    pub(in crate::transaction::sqlite_tx) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn split(&self) -> (TableName, VersionNumber, bool) {
        let parts: Vec<&str> = self.0.split("__").collect();
        assert_eq!(parts.len(), 3);
        (
            TableName::new(parts[0]).unwrap(),
            VersionNumber::from(parts[1].parse::<u64>().unwrap()),
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
