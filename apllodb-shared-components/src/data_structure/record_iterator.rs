pub(crate) mod record_field_ref_schema;

use std::{collections::VecDeque, sync::Arc};

use crate::{Record, SqlValues};

use self::record_field_ref_schema::RecordFieldRefSchema;

/// Iterator of [Record](crate::Record)s.
///
/// Note that Record is always generated from RecordIterator, who has ownership to [RecordFieldRefSchema](crate::RecordFieldRefSchema).
#[derive(Clone, PartialEq, Debug)]
pub struct RecordIterator {
    schema: Arc<RecordFieldRefSchema>,
    inner: VecDeque<SqlValues>,
}
impl RecordIterator {
    /// Constructor
    pub fn new<IntoValues: Into<SqlValues>, I: IntoIterator<Item = IntoValues>>(
        schema: RecordFieldRefSchema,
        it: I,
    ) -> Self {
        Self {
            schema: Arc::new(schema),
            inner: it
                .into_iter()
                .map(|into_values| into_values.into())
                .collect(),
        }
    }
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .pop_front()
            .map(|values| Record::new(self.schema.clone(), values))
    }
}
