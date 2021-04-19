use std::fmt::Display;

use apllodb_storage_engine_interface::ColumnName;
use serde::{Deserialize, Serialize};

/// Name of an attribute.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum AttributeName {
    /// Column
    ColumnNameVariant(ColumnName),
}

impl AttributeName {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            AttributeName::ColumnNameVariant(cn) => cn.as_str(),
        }
    }
}

impl Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
