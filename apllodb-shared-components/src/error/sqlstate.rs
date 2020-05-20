use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// SQLSTATE.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SqlState(String);

impl SqlState {
    pub fn new(sqlstate: String) -> Self {
        Self(sqlstate)
    }
}

impl Display for SqlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
