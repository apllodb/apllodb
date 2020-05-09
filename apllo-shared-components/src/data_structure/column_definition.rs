use serde::{Deserialize, Serialize};
use super::{data_type::DataType, column_name::ColumnName, column_constraints::ColumnConstraints};
use crate::error::AplloResult;

/// Column definition.
/// Note that NULLABLE SQL constraint is treated as DataType (not ColumnConstraint).
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDefinition {
    column_name: ColumnName,
    data_type: DataType,
    column_constraints: ColumnConstraints,
}
impl ColumnDefinition {
    /// Constructor
    pub fn new(
        column_name: ColumnName,
        data_type: DataType,
        column_constraints: ColumnConstraints,
    ) -> AplloResult<Self> {
        Ok(Self {
            column_name,
            data_type,
            column_constraints,
        })
    }

    /// Ref to ColumnName.
    pub fn column_name(&self) -> &ColumnName {
        &self.column_name
    }

    /// Ref to DataType.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}
