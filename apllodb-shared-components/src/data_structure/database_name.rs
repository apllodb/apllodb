use super::ShortName;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Database name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct DatabaseName(ShortName);

impl From<ShortName> for DatabaseName {
    fn from(name: ShortName) -> Self {
        Self(name)
    }
}

impl Display for DatabaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DatabaseName {
    pub fn as_short_name(&self) -> &ShortName {
        &self.0
    }
}
