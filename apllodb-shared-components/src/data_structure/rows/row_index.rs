use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::SchemaIndex;

/// Matcher to [TableColumnName](crate::TableColumnName).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct RowIndex {
    table: Option<String>,
    column: String,
}

impl SchemaIndex for RowIndex {
    fn new(prefix: Option<String>, attr: String) -> Self {
        Self {
            table: prefix,
            column: attr,
        }
    }

    fn prefix(&self) -> Option<&str> {
        self.table.as_ref().map(|s| s.as_str())
    }

    fn attr(&self) -> &str {
        &self.column
    }
}

impl Display for RowIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl From<&str> for RowIndex {
    fn from(s: &str) -> Self {
        SchemaIndex::from(s)
    }
}
