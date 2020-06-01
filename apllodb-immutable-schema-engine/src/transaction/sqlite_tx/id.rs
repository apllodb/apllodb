use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Transaction ID.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::transaction::sqlite_tx) struct SqliteTxId(u64);

impl SqliteTxId {
    pub(in crate::transaction::sqlite_tx) fn new() -> Self {
        // FIXME generate monotonically increasing number (current time might be the same for 2 callers).
        let now = Utc::now().timestamp_nanos() as u64;
        Self(now)
    }
}

impl std::fmt::Display for SqliteTxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
