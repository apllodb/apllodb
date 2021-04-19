use std::collections::HashSet;

use apllodb_shared_components::SchemaIndex;
use serde::{Deserialize, Serialize};

/// Projection query for single table columns.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum RowProjectionQuery {
    /// All columns in a table.
    /// Note that this variant cannot hold field/correlation alias.
    All,

    /// Some columns in a table.
    ColumnIndexes(HashSet<SchemaIndex>),
}
