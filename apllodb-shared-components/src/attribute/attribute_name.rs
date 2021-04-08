use crate::ColumnName;
use serde::{Deserialize, Serialize};

/// Name of an attribute.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum AttributeName {
    /// Column
    ColumnNameVariant(ColumnName),
}
