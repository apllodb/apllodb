use chrono::prelude::*;
use std::fmt::Display;

/// Transaction ID.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::sqlite) struct SqliteTxId {
    timestamp: DateTime<Utc>,
    thread_id: u64,
}

impl SqliteTxId {
    pub(in crate::sqlite) fn new() -> Self {
        let now = Utc::now();

        // FIXME Need Ord value which definitely differ even if `now` is the same.
        // ThreadId::as_u64() is nightly-only.
        // let thread_id = std::thread::current().id().as_u64();

        let thread_id = 0;

        Self {
            timestamp: now,
            thread_id,
        }
    }
}

impl Display for SqliteTxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SqliteTxId {{ timestamp: {:?}, thread_id: {} }}",
            self.timestamp, self.thread_id
        )
    }
}
