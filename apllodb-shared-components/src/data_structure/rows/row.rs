pub(crate) mod field_index;
pub(crate) mod record_pos;

use crate::{error::ApllodbResult, SqlConvertible, SqlValue, SqlValues};
use std::ops::Index;

use self::record_pos::RecordPos;

/// Primitive row representation used in storage engines and query processor
///
/// Clients do not directly use this struct but does [apllodb-server::Record](apllodb-server::Record) instead.
///
/// Row is meant to be read-only data, created while SELECT.
#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    values: SqlValues,
}

impl Row {
    /// Constructor
    pub fn new(values: SqlValues) -> Self {
        Self { values }
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
    pub fn get<T: SqlConvertible>(&self, pos: RecordPos) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(pos)?;
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
    pub fn get_sql_value(&self, pos: RecordPos) -> ApllodbResult<&SqlValue> {
        let sql_value = self.values.index(pos);
        Ok(sql_value)
    }

    /// Shrink a record into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(mut self, positions: &[RecordPos]) -> ApllodbResult<Self> {
        self.values = self.values.projection(&positions);
        Ok(self)
    }

    /// Get raw representation
    pub fn into_values(self) -> SqlValues {
        self.values
    }
}
