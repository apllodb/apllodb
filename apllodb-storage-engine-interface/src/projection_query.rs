use apllodb_shared_components::RecordFieldRefSchema;
use serde::{Deserialize, Serialize};

/// Projection query for single table columns.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum ProjectionQuery {
    /// All columns in a table.
    /// Note that this variant cannot hold field/correlation alias.
    All,

    /// Some columns in a table.
    Schema(RecordFieldRefSchema),
}
