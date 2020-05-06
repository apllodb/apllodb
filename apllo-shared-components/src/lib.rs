#![deny(
    warnings,
    // missing_docs,
    missing_debug_implementations)]

//! TBD
//!
//! まずは、TableNameとか、クエリプロセッサやストレージマネージャなどどの持ち物でもないstructを置いていく

pub mod error;

use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TableName(pub String);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(pub String);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub column_name: ColumnName,
    pub data_type: DataType,
    pub column_constraints: Vec<ColumnConstraint>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct DataType {
    pub kind: DataTypeKind,
    pub nullable: bool,
}

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
