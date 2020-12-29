use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::Record;

/// Iterator of [Record](crate::Record)s.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RecordIterator {
    // TODO use batched Records for memory reduction?
    inner: VecDeque<Record>,
}
impl RecordIterator {
    /// Constructor
    pub fn new<IntoRecord: Into<Record>, I: IntoIterator<Item = IntoRecord>>(it: I) -> Self {
        Self {
            inner: it
                .into_iter()
                .map(|into_record| into_record.into())
                .collect(),
        }
    }
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}
