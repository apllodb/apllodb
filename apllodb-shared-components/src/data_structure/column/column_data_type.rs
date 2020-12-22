use crate::data_structure::{ColumnDefinition, DataType};
use serde::{Deserialize, Serialize};

use super::ColumnReference;

/// Column with data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDataType {
    column: ColumnReference,
    data_type: DataType,
}

impl From<&ColumnDefinition> for ColumnDataType {
    fn from(d: &ColumnDefinition) -> Self {
        Self {
            column: d.column_ref().clone(),
            data_type: d.data_type().clone(),
        }
    }
}

impl ColumnDataType {
    pub fn new(column: ColumnReference, data_type: DataType) -> Self {
        Self { column, data_type }
    }

    /// Ref to column reference.
    pub fn column_ref(&self) -> &ColumnReference {
        &self.column
    }

    /// Ref to data type.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}
