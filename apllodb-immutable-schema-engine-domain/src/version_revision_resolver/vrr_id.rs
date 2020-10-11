use serde::{Deserialize, Serialize};

/// ID of Version-Revision Resolver's entry.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VRRId(u64);
