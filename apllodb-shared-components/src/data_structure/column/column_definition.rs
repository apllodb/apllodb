use super::ColumnDataType;
use crate::data_structure::{ColumnConstraints, ColumnName, DataType};
use crate::error::ApllodbResult;
use serde::{Deserialize, Serialize};

/// Column definition used in DDL.
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
    ) -> ApllodbResult<Self> {
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

    /// Ref to ColumnConstraints.
    pub fn column_constraints(&self) -> &ColumnConstraints {
        &self.column_constraints
    }

    /// Ref to ColumnDataType
    pub fn column_data_type(&self) -> ColumnDataType {
        ColumnDataType::new(self.column_name.clone(), self.data_type.clone())
    }
}
