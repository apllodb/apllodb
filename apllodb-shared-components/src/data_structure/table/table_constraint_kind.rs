use crate::data_structure::{ColumnDataType, ColumnName};
use serde::{Deserialize, Serialize};

/// A constraint parameter in a table definition.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum TableConstraintKind {
    /// PRIMARY KEY ({column_name}, ...)
    PrimaryKey {
        /// compound columns
        column_data_types: Vec<ColumnDataType>,
    },

    /// UNIQUE ({column_name}, ...)
    Unique {
        /// Compound columns.
        ///
        /// Since multiple unique keys can be applied to the same column,
        /// column data type master is not held here.
        column_names: Vec<ColumnName>,
    },
}
