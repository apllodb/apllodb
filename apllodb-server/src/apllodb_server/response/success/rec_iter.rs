// TODO remove in favor of sql-processor::Records

use std::sync::Arc;

use apllodb_shared_components::{RecordFieldRefSchema, Records};

use crate::Rec;

/// Iterator of [Rec](crate::Rec)s.
#[derive(Clone, PartialEq, Debug)]
pub struct RecIter {
    schema: Arc<RecordFieldRefSchema>,
    records: Records,
}

impl Iterator for RecIter {
    type Item = Rec;

    fn next(&mut self) -> Option<Self::Item> {
        self.records
            .next()
            .map(|record| Rec::new(self.schema.clone(), record))
    }
}

impl From<Records> for RecIter {
    fn from(records: Records) -> Self {
        let schema = { records.as_schema().clone() };
        Self {
            schema: Arc::new(schema),
            records,
        }
    }
}
