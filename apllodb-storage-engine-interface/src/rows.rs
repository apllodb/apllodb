pub(crate) mod row;
pub(crate) mod row_schema;

use std::collections::VecDeque;

use apllodb_shared_components::{ApllodbResult, RPos, Schema, SchemaIndex};

use self::{row::Row, row_schema::RowSchema};

/// Iterator of [Row](crate::Row)s with [RowSchema](crate::RowSchema).
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Rows {
    schema: RowSchema,
    inner: VecDeque<Row>,
}

impl Rows {
    /// Constructor
    pub fn new<IntoRow: Into<Row>, I: IntoIterator<Item = IntoRow>>(
        schema: RowSchema,
        it: I,
    ) -> Self {
        Self {
            schema,
            inner: it
                .into_iter()
                .map(|into_values| into_values.into())
                .collect(),
        }
    }

    /// Horizontally shrink records.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(self, indexes: &[SchemaIndex]) -> ApllodbResult<Self> {
        let new_schema = self.schema.projection(indexes)?;

        let projection_positions = indexes
            .iter()
            .map(|idx| {
                let (pos, _) = self.schema.index(idx)?;
                Ok(pos)
            })
            .collect::<ApllodbResult<Vec<RPos>>>()?;

        let new_inner: Vec<Row> = self
            .inner
            .into_iter()
            .map(|row| row.projection(&projection_positions))
            .collect();

        Ok(Self::new(new_schema, new_inner))
    }

    /// ref to schema
    pub fn as_schema(&self) -> &RowSchema {
        &self.schema
    }
}

impl Iterator for Rows {
    type Item = Row;

    fn next(&mut self) -> Option<Row> {
        self.inner.pop_front()
    }
}
