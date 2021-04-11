use std::fmt::Display;

use apllodb_storage_engine_interface::TableName;
use serde::{Deserialize, Serialize};

/// Name of a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum CorrelationName {
    /// Table name
    TableNameVariant(TableName),
}

impl Display for CorrelationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorrelationName::TableNameVariant(tn) => {
                write!(f, "{}", tn.as_str())
            }
        }
    }
}
