pub(crate) mod row;

use serde::{Deserialize, Serialize};

use crate::Row;

/// Iterator of [Row](crate::Row)s with [RowSchema](crate::RowSchema).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
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
        self.schema.as_ref()
    }
}
