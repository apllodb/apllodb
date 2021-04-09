pub(crate) mod row;
pub(crate) mod row_schema;

use serde::{Deserialize, Serialize};

use self::{row::Row, row_schema::RowSchema};

/// Iterator of [Row](crate::Row)s with [RowSchema](crate::RowSchema).
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Rows {
    schema: RowSchema,
    inner: Vec<Row>,
}

impl Rows {
    /// Constructor
    pub fn new<IntoRows: Into<Row>, I: IntoIterator<Item = IntoRows>>(
        schema: RowSchema,
        it: I,
    ) -> Self {
        Self {
            schema: schema,
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
