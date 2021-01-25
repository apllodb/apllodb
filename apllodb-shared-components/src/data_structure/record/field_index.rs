use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::ColumnReference;

/// Used to get a value from a record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum FieldIndex {
    /// column reference
    InColumnReference(ColumnReference),
}

impl Display for FieldIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FieldIndex::InColumnReference(colref) => colref.to_string(),
        };
        write!(f, "{}", s)
    }
}
