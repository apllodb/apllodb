use apllodb_shared_components::{SchemaIndex, SqlValue};
use serde::{Deserialize, Serialize};

/// Selection query for single table.
///
/// This struct is independent of physical column distribution (PK, secondary-index, row, ...).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum RowSelectionQuery {
    /// Full scan
    FullScan,

    /// Simple probe (e.g. `c1 = 777`)
    Probe {
        column: SchemaIndex,
        value: SqlValue,
    },
}
