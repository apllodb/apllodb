use apllodb_shared_components::data_structure::ColumnName;
use serde::{Deserialize, Serialize};

/// Projection query for single table column references.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum ProjectionQuery {
    /// All columns in a table.
    All,
    /// Some columns in a table.
    ColumnNames(Vec<ColumnName>),
}
