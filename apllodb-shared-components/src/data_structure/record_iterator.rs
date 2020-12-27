use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::Record;

/// Iterator of [Record](crate::Record)s.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RecordIterator {
    // TODO use batched Records for memory reduction?
    inner: VecDeque<Record>,
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}
