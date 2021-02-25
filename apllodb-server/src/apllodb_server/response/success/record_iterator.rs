use apllodb_shared_components::{Record, Records};

/// Iterator of [Record](apllodb-shared-components::Record)s.
#[derive(Clone, PartialEq, Debug)]
pub struct RecordIterator(Records);

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl From<Records> for RecordIterator {
    fn from(records: Records) -> Self {
        Self(records)
    }
}
