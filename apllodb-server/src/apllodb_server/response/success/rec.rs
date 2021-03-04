use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbResult, FieldIndex, Record, RecordFieldRefSchema, SqlConvertible,
};

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

    /// Get Rust value from rec field.
    ///
    /// Returns `None` for NULL.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb-shared-components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get<T: SqlConvertible>(&self, index: &FieldIndex) -> ApllodbResult<Option<T>> {
        let idx = self.schema.resolve_index(index)?;
        self.record.get(idx)
    }
}
