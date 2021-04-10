use apllodb_shared_components::{SchemaIndex, SchemaName};
use serde::{Deserialize, Serialize};

use crate::{column::column_name::ColumnName, table::table_name::TableName};

/// Full name in storage-engine: `TableName . ColumnName`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableColumnName {
    table: TableName,
    column: ColumnName,
}

impl SchemaName for TableColumnName {
    fn matches(&self, index: &SchemaIndex) -> bool {
        todo!()
    }
}
