use apllodb_shared_components::TableColumnName;

use crate::rows::row::Row;

impl Row {
    /// WARN: internal SqlValues might get different from RecordFieldRefSchema
    pub fn naive_join(self, right: Self) -> Self {
        for right_sql_value in right.into_values() {
            self.append(right_sql_value);
        }
        self
    }
}

impl TableColumnName {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(
            TableName::factory(table_name),
            ColumnName::factory(column_name),
        )
    }
}
