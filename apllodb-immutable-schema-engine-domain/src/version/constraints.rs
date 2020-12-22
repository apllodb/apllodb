use super::constraint_kind::VersionConstraintKind;
use serde::{Deserialize, Serialize};

/// Constraints that each record must satisfy.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(in crate::version) struct VersionConstraints {
    kinds: Vec<VersionConstraintKind>,
}

impl Default for VersionConstraints {
    fn default() -> Self {
        Self { kinds: vec![] }
    }
}
