use std::fmt::Display;

use serde::{Deserialize, Serialize};

use apllodb_shared_components::{
    ColumnName, CorrelationReference, FieldReference, FullFieldReference, TableName,
};

/// Table column reference == "table_name.column_name".
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableColumnReference {
    table_name: TableName,
    column_name: ColumnName,
}

impl Display for TableColumnReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.table_name.as_str(),
            self.column_name.as_str()
        )
    }
}

impl TableColumnReference {
    pub fn as_table_name(&self) -> &TableName {
        &self.table_name
    }

    pub fn as_column_name(&self) -> &ColumnName {
        &self.column_name
    }
}

impl From<TableColumnReference> for FullFieldReference {
    fn from(tcr: TableColumnReference) -> Self {
        let corr = CorrelationReference::TableNameVariant(tcr.table_name);
        let field = FieldReference::ColumnNameVariant(tcr.column_name);
        Self::new(corr, field)
    }
}
