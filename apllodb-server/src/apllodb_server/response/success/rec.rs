use std::sync::Arc;

use apllodb_shared_components::{Record, RecordFieldRefSchema};

/// Iterator of [Rec](crate::Rec)s.
#[derive(Clone, PartialEq, Debug)]
pub struct Rec {
    schema: Arc<RecordFieldRefSchema>,
    record: Record,
}

impl Rec {
    pub(crate) fn new(schema: Arc<RecordFieldRefSchema>, record: Record) -> Self {
        Self { schema, record }
    }
}
