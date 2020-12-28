pub(crate) mod field_index;

use crate::{
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    traits::sql_convertible::SqlConvertible,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use self::field_index::FieldIndex;

use super::value::sql_value::SqlValue;

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, new)]
pub struct Record {
    fields: HashMap<FieldIndex, SqlValue>,
}

impl Record {
    /// Get value from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    /// - Errors from [SqlValue::unpack()](x.html).
    pub fn get<T: SqlConvertible>(&self, index: FieldIndex) -> ApllodbResult<T> {
        let sql_value = self.fields.get(&index).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("invalid field reference: `{:?}`", index),
                None,
            )
        })?;
        Ok(sql_value.unpack()?)
    }

    /// Shrink a record into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(mut self, fields: &HashSet<FieldIndex>) -> ApllodbResult<Self> {
        if let Some(invalid_field) = fields
            .difference(&self.fields.keys().cloned().collect())
            .next()
        {
            return Err(ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("invalid field reference: `{:?}`", invalid_field),
                None,
            ));
        }

        let new_fields: HashMap<FieldIndex, SqlValue> = self
            .fields
            .drain()
            .filter(|(k, _)| fields.contains(k))
            .collect();
        self.fields = new_fields;

        Ok(self)
    }
}
