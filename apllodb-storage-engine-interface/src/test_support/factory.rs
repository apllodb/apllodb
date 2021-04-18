use apllodb_shared_components::{test_support::factory::random_id, SqlType};

use crate::{
    column::{column_data_type::ColumnDataType, column_name::ColumnName},
    table::table_name::TableName,
    table_column_name::TableColumnName,
};

impl TableName {
    /// randomly generate a table name
    pub fn random() -> Self {
        Self::new(random_id()).unwrap()
    }
}

impl TableName {
    pub fn factory(table_name: &str) -> Self {
        Self::new(table_name).unwrap()
    }
}

impl ColumnName {
    pub fn factory(column_name: &str) -> Self {
        Self::new(column_name).unwrap()
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

impl ColumnDataType {
    pub fn factory(column_name: &str, sql_type: SqlType, nullable: bool) -> Self {
        Self::new(ColumnName::factory(column_name), sql_type, nullable)
    }
}
