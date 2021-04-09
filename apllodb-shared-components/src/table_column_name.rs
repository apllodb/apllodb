use crate::{ColumnName, SchemaIndex, SchemaName, TableName};
use serde::{Deserialize, Serialize};

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
