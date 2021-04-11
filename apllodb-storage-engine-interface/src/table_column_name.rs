use apllodb_shared_components::SchemaName;
use serde::{Deserialize, Serialize};

use crate::{column::column_name::ColumnName, table::table_name::TableName};

/// Full name in storage-engine: `TableName . ColumnName`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableColumnName {
    table: TableName,
    column: ColumnName,
}

impl SchemaName for TableColumnName {
    fn _attr_matches(&self, attr: &str) -> bool {
        self.column.as_str() == attr
    }

    fn _prefix_attr_match(&self, prefix: &str, attr: &str) -> bool {
        self.table.as_str() == prefix && self.column.as_str() == attr
    }
}

impl TableColumnName {
    /// ref to table name
    pub fn as_table_name(&self) -> &TableName {
        &self.table
    }

    /// ref to column name
    pub fn as_column_name(&self) -> &ColumnName {
        &self.column
    }
}
