use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Session ID.
///
/// It is a server's responsibility to generate underlying u64 ID.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SessionId(u64);

impl SessionId {
    /// Generate session ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
