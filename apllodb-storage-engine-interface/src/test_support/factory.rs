use apllodb_shared_components::{ColumnName, TableName};

use crate::TableColumnReference;

impl TableColumnReference {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(
            TableName::factory(table_name),
            ColumnName::factory(column_name),
        )
    }
}
