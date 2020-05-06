use serde::{Deserialize, Serialize};

/// Constraints that set of record (not each record) must satisfy.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(super) enum VersionSetConstraint {
    Unique,
    PrimaryKey,
}
