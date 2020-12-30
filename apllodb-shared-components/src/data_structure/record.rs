pub(crate) mod field_index;

use crate::{
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::field_index::FieldIndex;

use super::value::sql_value::SqlValue;

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, new)]
pub struct Record {
    fields: HashMap<FieldIndex, SqlValue>,
}

impl Record {
    /// Get Rust value from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    /// - Errors from [SqlValue::unpack()](x.html).
    pub fn get<T: SqlConvertible>(&self, index: &FieldIndex) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(index)?;
        Ok(sql_value.unpack()?)
    }

    /// Get [SqlValue](crate::SqlValue) from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get_sql_value(&self, index: &FieldIndex) -> ApllodbResult<&SqlValue> {
        let sql_value = self.fields.get(index).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("invalid field reference: `{:?}`", index),
                None,
            )
        })?;
        Ok(sql_value)
    }

    /// Get raw representation
    pub fn into_field_values(self) -> HashMap<FieldIndex, SqlValue> {
        self.fields
    }
}
