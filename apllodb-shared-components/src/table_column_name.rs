use crate::{ColumnName, SchemaName, TableName, data_structure::rows::{row_index::RowIndex, row_schema::RowSchema}};
use serde::{Deserialize, Serialize};

/// Full name in storage-engine: `TableName . ColumnName`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableColumnName {
    table: TableName,
    column: ColumnName,
}

impl SchemaName<RowSchema> for TableColumnName {
    fn matches(&self, index: &RowIndex) -> bool {
        todo!()
    }
}
