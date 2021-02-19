use std::collections::HashMap;

use apllodb_shared_components::{AliasName, ColumnName, FullFieldReference, TableWithAlias};
use serde::{Deserialize, Serialize};

/// Alias to a table and columns given from SQL.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AliasDef {
    table_alias: Option<AliasName>,
    column_aliases: HashMap<ColumnName, AliasName>,
}

impl AliasDef {
    pub fn new(
        table_with_alias: TableWithAlias,
        full_field_references: &[FullFieldReference],
    ) -> Self {
        let column_aliases: HashMap<ColumnName, AliasName> = full_field_references
            .iter()
            .filter_map(|ffr| {
                let fr = ffr.as_field_reference();
                fr.as_alias_name()
                    .map(|alias| (fr.as_column_name().clone(), alias.clone()))
            })
            .collect();

        Self {
            table_alias: table_with_alias.alias,
            column_aliases,
        }
    }

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
