use apllo_shared_components::{ColumnName, DataType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VersionColumnDefinition {
    column: ColumnName,
    data_type: DataType,
    constraints: Vec<VersionConstraint>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VersionConstraint {
    kinds: Vec<VersionConstraintKind>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum VersionConstraintKind {
    Default(/* TODO: Expr */),
    Check(/* TODO: Expr */),
    ForeignKey(/* TODO: ??? */),
}
