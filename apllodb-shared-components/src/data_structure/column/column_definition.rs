use serde::{Deserialize, Serialize};

use crate::data_structure::data_type::DataType;

use super::{
    column_constraints::ColumnConstraints, column_data_type::ColumnDataType,
    column_reference::ColumnReference,
};

/// Column definition used in DDL.
/// Note that NULLABLE SQL constraint is treated as [DataType](crate::DataType) (not [ColumnConstraints](crate::ColumnConstraints)).
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnDefinition {
    column_ref: ColumnReference,
    data_type: DataType,
    column_constraints: ColumnConstraints,
}
impl ColumnDefinition {
    /// Ref to ColumnReference.
    pub fn column_ref(&self) -> &ColumnReference {
        &self.column_ref
    }

    /// Ref to DataType.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Ref to ColumnConstraints.
    pub fn column_constraints(&self) -> &ColumnConstraints {
        &self.column_constraints
    }

    /// ColumnDataType
    pub fn column_data_type(&self) -> ColumnDataType {
        ColumnDataType::new(self.column_ref.clone(), self.data_type.clone())
    }
}
