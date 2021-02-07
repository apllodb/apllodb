use std::collections::HashMap;

use apllodb_shared_components::{AliasName, ColumnName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AliasDef {
    table_alias: Option<AliasName>,
    column_aliases: HashMap<ColumnName, AliasName>,
}

impl AliasDef {
    /// set table alias
    pub fn set_table_alias(mut self, table_alias: AliasName) -> Self {
        self.table_alias = Some(table_alias);
        self
    }

    /// add column alias
    pub fn add_column_alias(mut self, column_name: ColumnName, column_alias: AliasName) -> Self {
        self.column_aliases.insert(column_name, column_alias);
        self
    }
}
