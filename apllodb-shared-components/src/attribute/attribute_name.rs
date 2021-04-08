use std::fmt::Display;

use crate::ColumnName;
use serde::{Deserialize, Serialize};

/// Name of an attribute.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum AttributeName {
    /// Column
    ColumnNameVariant(ColumnName),
}

impl Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeName::ColumnNameVariant(cn) => {
                write!(f, "{}", cn.as_str())
            }
        }
    }
}
