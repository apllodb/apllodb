mod field_index;

pub use field_index::FieldIndex;

use super::{SqlConvertible, SqlValue};
use crate::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Record {
    fields: HashMap<FieldIndex, SqlValue>,
}

impl Record {
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
