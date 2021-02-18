use std::fmt::Display;

use crate::TableWithAlias;
use serde::{Deserialize, Serialize};

/// Reference to a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CorrelationReference {
    /// table
    TableVariant(TableWithAlias),
    // TODO SubQueryAliasVariant { ... }
}

impl Display for CorrelationReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CorrelationReference::TableVariant(table) => table.to_string(),
        };
        write!(f, "{}", s)
    }
}
