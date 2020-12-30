use serde::{Deserialize, Serialize};

use crate::ColumnReference;

/// Used to get a value from a record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum FieldIndex {
    /// column reference
    InColumnReference(ColumnReference),
}
