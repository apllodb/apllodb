use serde::{Deserialize, Serialize};

use crate::{data_structure::value::sql_value::SqlValue, ColumnName};

/// Column value with ColumnReference
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ColumnValue {
    column_name: ColumnName,
    value: SqlValue,
}

impl ColumnValue {
    /// Ref to column name
    pub fn as_column_name(&self) -> &ColumnName {
        &self.column_name
    }

    /// Into SQL value
    pub fn into_sql_value(self) -> SqlValue {
        self.value
    }
}
