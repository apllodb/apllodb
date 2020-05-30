use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Transaction ID.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::transaction::simple_tx) struct SimpleTxId(u64);

impl SimpleTxId {
    pub(in crate::transaction::simple_tx) fn new() -> Self {
        // FIXME generate monotonically increasing number (current time might be the same for 2 callers).
        let now = Utc::now().timestamp_nanos() as u64;
        Self(now)
    }
}
