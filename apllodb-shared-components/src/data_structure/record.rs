pub(crate) mod field_index;

use crate::{
    error::ApllodbResult, traits::sql_convertible::SqlConvertible, FieldIndex, FullFieldReference,
    SqlValues,
};
use std::{ops::Index, sync::Arc};

use super::{
    record_iterator::record_field_ref_schema::RecordFieldRefSchema, value::sql_value::SqlValue,
};

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
///
/// Record is meant to be read-only data.
/// It is created while SELECT by a storage engine or query processor.
#[derive(Clone, PartialEq, Debug)]
pub struct Record {
    schema: Arc<RecordFieldRefSchema>,
    values: SqlValues,
}

impl Record {
    /// Constructor
    pub fn new(schema: Arc<RecordFieldRefSchema>, values: SqlValues) -> Self {
        Self { schema, values }
    }

    /// Get Rust value from record's field.
    ///
    /// Returns `None` if matching [SqlValue](crate::SqlValue) is NULL.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    /// - Errors from [SqlValue::unpack()](x.html).
    pub fn get<T: SqlConvertible>(&self, index: &FieldIndex) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(index)?;
        let ret = match sql_value {
            SqlValue::Null => None,
            SqlValue::NotNull(nn) => Some(nn.unpack()?),
        };
        Ok(ret)
    }

    /// Get [SqlValue](crate::SqlValue) from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get_sql_value(&self, index: &FieldIndex) -> ApllodbResult<&SqlValue> {
        let idx = self.schema.resolve_index(index)?;
        let sql_value = self.values.index(idx);
        Ok(sql_value)
    }

    /// Shrink a record into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(mut self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let idxs: Vec<usize> = projection
            .iter()
            .map(|index| self.schema.resolve_index(index))
            .collect::<ApllodbResult<_>>()?;

        self.values = self.values.projection(&idxs);

        Ok(self)
    }

    /// Joins another record after this record.
    pub fn join(mut self, right: Self) -> Self {
        self.schema = Arc::new(self.schema.joined(right.schema()));
        self.values.join(right.values);
        self
    }

    /// Get raw representation
    pub fn into_values(self) -> SqlValues {
        self.values
    }

    /// Get raw representation
    pub fn into_ffr_vals(self) -> Vec<(FullFieldReference, SqlValue)> {
        self.schema
            .as_full_field_references()
            .iter()
            .cloned()
            .zip(self.values)
            .collect()
    }

    /// ref to schema
    pub fn schema(&self) -> &RecordFieldRefSchema {
        self.schema.as_ref()
    }
}
