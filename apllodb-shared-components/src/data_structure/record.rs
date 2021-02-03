pub(crate) mod field_index;

use crate::{
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
    Expression, FieldIndex, FullFieldReference,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
};

use super::value::sql_value::SqlValue;

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, new)]
pub struct Record {
    fields: HashMap<FullFieldReference, SqlValue>,
}

impl Record {
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
        let ffr = FullFieldReference::try_from(index.clone())?;
        let sql_value = self.fields.get(&ffr).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("invalid field reference: `{:?}`", index),
                None,
            )
        })?;
        Ok(sql_value)
    }

    /// Shrink a record into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(mut self, projection: &HashSet<FieldIndex>) -> ApllodbResult<Self> {
        let projection: HashSet<FullFieldReference> = projection
            .iter()
            .cloned()
            .map(FullFieldReference::try_from)
            .collect::<ApllodbResult<_>>()?;

        if let Some(invalid_field) = projection
            .difference(&self.fields.keys().cloned().collect())
            .next()
        {
            return Err(ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("invalid field reference: `{:?}`", invalid_field),
                None,
            ));
        }

        let new_fields: HashMap<FullFieldReference, SqlValue> = self
            .fields
            .drain()
            .filter(|(k, _)| projection.contains(k))
            .collect();
        self.fields = new_fields;

        Ok(self)
    }

    /// Check if whether this record satisfies selection condition.
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - `condition` is not evaluated as BOOLEAN type.
    pub fn selection(&self, _condition: &Expression) -> ApllodbResult<bool> {
        todo!()
    }

    /// Joins another record into this record.
    ///
    /// # Failures
    ///
    /// - [DuplicateObject](crate::ApllodbErrorKind::DuplicateObject) when:
    ///   - `another` has the same field with self.
    pub fn join(mut self, mut another: Record) -> ApllodbResult<Self> {
        let another_ffr: HashSet<&FullFieldReference> = another.fields.keys().collect();
        if let Some(dup_field) = self.fields.keys().find(|field| another_ffr.contains(field)) {
            return Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "joining two records with duplicate field: `{:?}`",
                    dup_field
                ),
                None,
            ));
        }

        let new_fields: HashMap<FullFieldReference, SqlValue> =
            self.fields.drain().chain(another.fields.drain()).collect();
        self.fields = new_fields;

        Ok(self)
    }

    /// Get raw representation
    pub fn into_field_values(self) -> HashMap<FullFieldReference, SqlValue> {
        self.fields
    }
}
