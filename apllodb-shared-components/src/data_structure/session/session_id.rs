use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Session ID.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SessionId(u64);

impl Default for SessionId {
    /// Generate session ID.
    fn default() -> Self {
        let r = fastrand::u64(..);
        Self(r)
    }
}
