use serde::{Deserialize, Serialize};

/// Version number.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VersionNumber(u64);

impl VersionNumber {
    pub(crate) fn initial() -> Self {
        Self(1)
    }

    pub(crate) fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}
