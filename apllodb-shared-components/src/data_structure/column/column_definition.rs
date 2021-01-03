use serde::{Deserialize, Serialize};

use super::{column_constraints::ColumnConstraints, column_data_type::ColumnDataType};

/// Column definition used in DDL.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnDefinition {
    column_data_type: ColumnDataType,
    column_constraints: ColumnConstraints,
}
impl ColumnDefinition {
    /// ColumnDataType
    pub fn column_data_type(&self) -> &ColumnDataType {
        &self.column_data_type
    }

    /// Ref to ColumnConstraints.
    pub fn column_constraints(&self) -> &ColumnConstraints {
        &self.column_constraints
    }
}
