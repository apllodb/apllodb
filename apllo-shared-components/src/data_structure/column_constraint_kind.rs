use serde::{Deserialize, Serialize};
use super::column_name::ColumnName;

/// A constraint parameter in a column definition.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ColumnConstraintKind {
    /// {column_name} {DATA_TYPE} PRIMARY KEY
    PrimaryKey { column_name: ColumnName },

    /// {column_name} {DATA_TYPE} UNIQUE
    Unique { column_name: ColumnName },
}
