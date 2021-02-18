use std::collections::HashMap;

use apllodb_shared_components::{AliasName, ColumnName};
use serde::{Deserialize, Serialize};

/// Alias to a table and columns given from SQL.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AliasDef {
    table_alias: Option<AliasName>,
    column_aliases: HashMap<ColumnName, AliasName>,
}

impl AliasDef {
    /// get table alias
    pub fn table_alias(&self) -> Option<&AliasName> {
        self.table_alias.as_ref()
    }

    /// get column alias
    pub fn column_aliases(&self) -> &HashMap<ColumnName, AliasName> {
        &self.column_aliases
    }

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
