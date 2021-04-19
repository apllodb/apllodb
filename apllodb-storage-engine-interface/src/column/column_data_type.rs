use apllodb_shared_components::SqlType;
use serde::{Deserialize, Serialize};

use super::{column_definition::ColumnDefinition, column_name::ColumnName};

/// Column with data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnDataType {
    column: ColumnName,
    sql_type: SqlType,
    nullable: bool,
}

impl From<&ColumnDefinition> for ColumnDataType {
    fn from(d: &ColumnDefinition) -> Self {
        d.column_data_type().clone()
    }
}

impl ColumnDataType {
    /// Ref to column name.
    pub fn column_name(&self) -> &ColumnName {
        &self.column
    }

    /// Ref to data type.
    pub fn sql_type(&self) -> &SqlType {
        &self.sql_type
    }

    /// IS NULL
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
