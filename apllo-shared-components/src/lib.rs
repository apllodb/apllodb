#![deny(warnings, 
    //missing_docs,
    missing_debug_implementations)]

//! Data structures shared with multiple crates in the apllo workspace.

pub mod error;

use serde::{Deserialize, Serialize};

/// Table name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TableName(pub String);

/// Column name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(pub String);

/// Column definition.
/// Note that NULLABLE SQL constraint is treated as DataType (not ColumnConstraint).
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub column_name: ColumnName,
    pub data_type: DataType,
    pub column_constraints: Vec<ColumnConstraint>,
}

/// Data type.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct DataType {
    pub kind: DataTypeKind,
    pub nullable: bool,
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
    pub column_name: ColumnName,
    pub kind: ColumnConstraintKind,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ColumnConstraintKind {}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct TableConstraint {}
