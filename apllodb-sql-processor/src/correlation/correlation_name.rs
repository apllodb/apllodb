use std::fmt::Display;

use apllodb_storage_engine_interface::TableName;
use serde::{Deserialize, Serialize};

/// Name of a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum CorrelationName {
    /// Table name
    TableNameVariant(TableName),
}

impl CorrelationName {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            CorrelationName::TableNameVariant(tn) => tn.as_str(),
        }
    }
}

impl Display for CorrelationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
