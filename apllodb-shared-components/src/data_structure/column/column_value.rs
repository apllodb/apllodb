use serde::{Deserialize, Serialize};

use crate::data_structure::value::sql_value::SqlValue;

use super::column_reference::ColumnReference;

/// Column value with ColumnReference
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnValue {
    colref: ColumnReference,
    value: SqlValue,
}

impl ColumnValue {
    /// Ref to column reference
    pub fn as_column_ref(&self) -> &ColumnReference {
        &self.colref
    }

    /// Into SQL value
    pub fn into_sql_value(self) -> SqlValue {
        self.value
    }
}
