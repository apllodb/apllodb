pub(crate) mod record_field_ref_schema;

use std::{collections::VecDeque, sync::Arc};

use crate::{ApllodbResult, FieldIndex, FullFieldReference, Record, SqlValues};

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

    /// get FullFieldReferences
    pub fn as_full_field_references(&self) -> &[FullFieldReference] {
        self.schema.as_full_field_references()
    }

    /// ref to schema
    pub fn as_schema(&self) -> &RecordFieldRefSchema {
        self.schema.as_ref()
    }

    /// makes SqlValues
    pub fn into_sql_values(self) -> Vec<SqlValues> {
        self.inner.into_iter().collect()
    }

    /// Shrink records into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let projection_idxs = projection
            .iter()
            .map(|index| self.schema.resolve_index(index))
            .collect::<ApllodbResult<Vec<usize>>>()?;

        let new_schema = self.schema.projection(projection)?;

        let new_inner: VecDeque<SqlValues> = self
            .inner
            .into_iter()
            .map(|sql_values| sql_values.projection(&projection_idxs))
            .collect();

        Ok(Self::new(new_schema, new_inner))
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
