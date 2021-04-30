pub(crate) mod row;
pub(crate) mod row_schema;

use std::collections::VecDeque;

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
