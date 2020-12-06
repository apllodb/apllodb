use serde::{Deserialize, Serialize};

use crate::data_structure::SqlValue;

use super::ColumnReference;

/// Column value with ColumnReference
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnValue {
    colref: ColumnReference,
    value: SqlValue,
}

impl ColumnValue {
    //     /// Constructor.
    //     pub fn new(colref: ColumnReference, value: SqlValue) -> Self {
    //         Self { colref, value }
    //     }

    /// Ref to column reference
    pub fn as_column_ref(&self) -> &ColumnReference {
        &self.colref
    }

    //     /// Ref to SQL value
    //     pub fn as_sql_value(&self) -> &SqlValue {
    //         &self.value
    //     }

    /// Into SQL value
    pub fn into_sql_value(self) -> SqlValue {
        self.value
    }
}
