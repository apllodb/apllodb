use crate::VersionNumber;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) struct SqliteTableNameForVersion(String);

impl<S: Into<String>> From<S> for SqliteTableNameForVersion {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl SqliteTableNameForVersion {
    pub(crate) fn new(
        table_name: &TableName,
        version_number: &VersionNumber,
        activation_kind: ActivationKind,
    ) -> Self {
        let s = format!(
            "{}__{}__{}",
            table_name.to_sql_string(),
            version_ref.0,
            activation_kind.to_string()
        );

        // SQLite3 で ALTER TABLE xxx RENAME TO yyy; すると、テーブル名が勝手にダブルクオートされる件の対応:
        // https://darwin-edu.slack.com/archives/CTHGWT9PF/p1587962374045000
        // TODO: adhocすぎる実装なのでちゃんと考える
        let s = s.trim_matches('"');

        Self(s.into())
    }

    pub(crate) fn to_table_name(&self) -> TableName {
        self.split().0
    }
    pub(crate) fn to_version_ref(&self) -> VersionRef {
        self.split().1
    }
    pub(crate) fn is_active(&self) -> bool {
        self.split().2.is_active()
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn split(&self) -> (TableName, VersionRef, ActivationKind) {
        let parts: Vec<&str> = self.0.split("__").collect();
        assert_eq!(parts.len(), 3);
        (
            TableName::from(parts[0]),
            VersionRef(parts[1].parse().unwrap()),
            ActivationKind::from(parts[2]),
        )
    }
}
