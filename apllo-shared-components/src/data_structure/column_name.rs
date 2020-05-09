use serde::{Deserialize, Serialize};
use std::fmt::Display;
use super::ShortName;

/// Column name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(ShortName);

impl From<ShortName> for ColumnName {
    fn from(name: ShortName) -> Self {
        Self(name)
    }
}

impl Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
