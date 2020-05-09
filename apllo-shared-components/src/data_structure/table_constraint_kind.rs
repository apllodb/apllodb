use super::column_name::ColumnName;
use serde::{Deserialize, Serialize};

/// A constraint parameter in a table definition.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum TableConstraintKind {
    /// PRIMARY KEY ({column_name}, ...)
    PrimaryKey { column_names: Vec<ColumnName> },

    /// UNIQUE ({column_name}, ...)
    Unique { column_names: Vec<ColumnName> },
}
