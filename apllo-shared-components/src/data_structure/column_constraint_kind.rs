use serde::{Deserialize, Serialize};

/// A constraint parameter in a column definition.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ColumnConstraintKind {
    /// {column_name} {DATA_TYPE} PRIMARY KEY
    PrimaryKey,

    /// {column_name} {DATA_TYPE} UNIQUE
    Unique,
}
