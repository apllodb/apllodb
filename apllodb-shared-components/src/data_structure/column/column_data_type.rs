use crate::data_structure::{ColumnDefinition, ColumnName, DataType};
use serde::{Deserialize, Serialize};

/// Column with data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDataType {
    column: ColumnName,
    data_type: DataType,
}

impl From<&ColumnDefinition> for ColumnDataType {
    fn from(d: &ColumnDefinition) -> Self {
        Self {
            column: d.column_name().clone(),
            data_type: d.data_type().clone(),
        }
    }
}

impl ColumnDataType {
    pub fn new(column: ColumnName, data_type: DataType) -> Self {
        Self { column, data_type }
    }

    /// Ref to column name.
    pub fn column_name(&self) -> &ColumnName {
        &self.column
    }

    /// Ref to data type.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}
