use super::Version;
use serde::{Deserialize, Serialize};

/// Inactive Version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct InactiveVersion(Version);
