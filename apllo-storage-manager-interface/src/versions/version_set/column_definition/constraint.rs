use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VersionSetConstraint {
    kinds: Vec<VersionSetConstraintKind>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum VersionSetConstraintKind {
    Unique,
    PrimaryKey,
}
