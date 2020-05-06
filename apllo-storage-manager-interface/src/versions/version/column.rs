use apllo_shared_components::{ColumnDefinition, ColumnName, DataType};
use serde::{Deserialize, Serialize};

/// Column with data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(crate) struct ColumnDataType {
    column: ColumnName,
    data_type: DataType,
}

impl From<ColumnDefinition> for ColumnDataType {
    fn from(d: ColumnDefinition) -> Self {
        Self {
            column: d.column_name,
            data_type: d.data_type,
        }
    }
}
