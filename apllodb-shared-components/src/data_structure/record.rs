mod field_index;

pub use field_index::FieldIndex;

use super::SqlValue;
use crate::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use crate::traits::SqlConvertible;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Record {
    fields: HashMap<FieldIndex, SqlValue>,
}

impl Record {
    /// Get value from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](x.html) when:
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
}
