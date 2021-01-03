use serde::{Deserialize, Serialize};

use crate::data_structure::value::sql_type::SqlType;

use super::{column_definition::ColumnDefinition, column_reference::ColumnReference};

/// Column with data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnDataType {
    column: ColumnReference,
    sql_type: SqlType,
    nullable: bool,
}

impl From<&ColumnDefinition> for ColumnDataType {
    fn from(d: &ColumnDefinition) -> Self {
        d.column_data_type().clone()
    }
}

impl ColumnDataType {
    /// Ref to column reference.
    pub fn column_ref(&self) -> &ColumnReference {
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
