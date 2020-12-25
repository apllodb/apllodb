use serde::{Deserialize, Serialize};

use crate::data_structure::table::table_name::TableName;

use super::column_name::ColumnName;

/// Column reference == "correlation.column_name".
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnReference {
    table_name: TableName, // TODO correlation name (including `table1` in "FROM T as table1").
    column_name: ColumnName,
}

impl ColumnReference {
    /// Ref to table name
    pub fn as_table_name(&self) -> &TableName {
        &self.table_name
    }

    /// Ref to column name
    pub fn as_column_name(&self) -> &ColumnName {
        &self.column_name
    }
}
