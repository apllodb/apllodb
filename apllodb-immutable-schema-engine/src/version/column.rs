use apllodb_shared_components::data_structure::{ColumnDefinition, ColumnName, DataType};
use serde::{Deserialize, Serialize};

/// Column with data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(crate) struct ColumnDataType {
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
    /// Ref to column name.
    pub(crate) fn column_name(&self) -> &ColumnName {
        &self.column
    }

    /// Ref to data type.
    pub(crate) fn data_type(&self) -> &DataType {
        &self.data_type
    }
}
