use crate::data_structure::TableName;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::ColumnName;

/// Column reference == "correlation.column_name".
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnReference {
    table_name: TableName, // TODO correlation name (including `table1` in "FROM T as table1").
    column_name: ColumnName,
}

impl Display for ColumnReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.table_name, self.column_name)
    }
}

impl ColumnReference {
    /// Constructor.
    pub fn new(table_name: TableName, column_name: ColumnName) -> Self {
        Self {
            table_name,
            column_name,
        }
    }

    /// Ref to table name
    pub fn as_table_name(&self) -> &TableName {
        &self.table_name
    }

    /// Ref to column name
    pub fn as_column_name(&self) -> &ColumnName {
        &self.column_name
    }
}
