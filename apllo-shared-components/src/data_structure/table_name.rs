use super::validation_helper::names::ShortName;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Table name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TableName(ShortName);

impl From<ShortName> for TableName {
    fn from(name: ShortName) -> Self {
        Self(name)
    }
}

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
