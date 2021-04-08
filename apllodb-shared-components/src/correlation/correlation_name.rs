use crate::TableName;
use serde::{Deserialize, Serialize};

/// Name of a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CorrelationName {
    /// Table name
    TableNameVariant(TableName),
}
