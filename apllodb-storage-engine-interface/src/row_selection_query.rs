use serde::{Deserialize, Serialize};

use crate::SingleTableCondition;

/// Selection query for single table.
///
/// This struct is independent of physical column distribution (PK, secondary-index, row, ...).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum RowSelectionQuery {
    /// Full scan
    FullScan,

    /// WHERE condition for a single table.
    Condition(SingleTableCondition),
}
