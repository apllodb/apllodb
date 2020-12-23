use serde::{Deserialize, Serialize};

use crate::data_structure::column::column_name::ColumnName;

/// A constraint parameter in a table definition.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum TableConstraintKind {
    /// PRIMARY KEY ({column_name}, ...)
    PrimaryKey {
        /// Compound columns.
        column_names: Vec<ColumnName>,
    },

    /// UNIQUE ({column_name}, ...)
    Unique {
        /// Compound columns.
        column_names: Vec<ColumnName>,
    },
}
