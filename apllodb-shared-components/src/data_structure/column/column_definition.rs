use super::{ColumnDataType, ColumnReference};
use crate::data_structure::{ColumnConstraints, DataType};
use crate::error::ApllodbResult;
use serde::{Deserialize, Serialize};

/// Column definition used in DDL.
/// Note that NULLABLE SQL constraint is treated as DataType (not ColumnConstraint).
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDefinition {
    column_ref: ColumnReference,
    data_type: DataType,
    column_constraints: ColumnConstraints,
}
impl ColumnDefinition {
    /// Constructor
    pub fn new(
        column_ref: ColumnReference,
        data_type: DataType,
        column_constraints: ColumnConstraints,
    ) -> ApllodbResult<Self> {
        Ok(Self {
            column_ref,
            data_type,
            column_constraints,
        })
    }

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
