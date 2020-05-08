#![deny(warnings, 
    //missing_docs,
    missing_debug_implementations)]

//! Data structures shared with multiple crates in the apllo workspace.

pub mod error;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Table name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TableName(String);

/// Column name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(String);
impl Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Column definition.
/// Note that NULLABLE SQL constraint is treated as DataType (not ColumnConstraint).
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDefinition {
    column_name: ColumnName,
    data_type: DataType,
    column_constraints: Vec<ColumnConstraint>,
}
impl ColumnDefinition {
    pub fn column_name(&self) -> &ColumnName {
        &self.column_name
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}

/// Data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct DataType {
     kind: DataTypeKind,
     nullable: bool,
}

/// Data type kind.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum DataTypeKind {
    SmallInt,
    Integer,
    BigInt,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnConstraint {
     column_name: ColumnName,
     kind: ColumnConstraintKind,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ColumnConstraintKind {}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct TableConstraint {}
