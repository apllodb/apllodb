use super::{Version, VersionNumber};
use serde::{Deserialize, Serialize};

/// Inactive Version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct InactiveVersion(Version);

impl InactiveVersion {
    #[allow(dead_code)]
    /// Version number.
    pub fn number(&self) -> &VersionNumber {
        &self.0.number
    }
}
