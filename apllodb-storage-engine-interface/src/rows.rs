pub(crate) mod row;
pub(crate) mod row_schema;

use apllodb_shared_components::{ApllodbResult, RPos, Schema, SchemaIndex};
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
