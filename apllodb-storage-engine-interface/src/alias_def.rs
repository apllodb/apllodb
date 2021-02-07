use std::collections::HashMap;

use apllodb_shared_components::{
    AliasName, ColumnName, CorrelationReference, FieldReference, FullFieldReference,
};
use serde::{Deserialize, Serialize};

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

impl From<Vec<FullFieldReference>> for AliasDef {
    fn from(ffrs: Vec<FullFieldReference>) -> Self {
        let mut alias_def = Self::default();

        for ffr in ffrs {
            match (ffr.as_correlation_reference(), ffr.as_field_reference()) {
                (
                    CorrelationReference::TableNameVariant(_),
                    FieldReference::ColumnNameVariant(_),
                ) => {}
                (
                    CorrelationReference::TableNameVariant(_),
                    FieldReference::ColumnAliasVariant {
                        alias_name,
                        column_name,
                    },
                ) => {
                    alias_def = alias_def.add_column_alias(column_name.clone(), alias_name.clone())
                }
                (
                    CorrelationReference::TableAliasVariant { alias_name, .. },
                    FieldReference::ColumnNameVariant(_),
                ) => {
                    assert!(
                        alias_def.table_alias.is_none()
                            || alias_def.table_alias() == Some(alias_name)
                    );
                    alias_def = alias_def.set_table_alias(alias_name.clone());
                }
                (
                    CorrelationReference::TableAliasVariant {
                        alias_name: table_alias,
                        ..
                    },
                    FieldReference::ColumnAliasVariant {
                        alias_name: column_alias,
                        column_name,
                    },
                ) => {
                    assert!(
                        alias_def.table_alias.is_none()
                            || alias_def.table_alias() == Some(table_alias)
                    );
                    alias_def = alias_def
                        .set_table_alias(table_alias.clone())
                        .add_column_alias(column_name.clone(), column_alias.clone());
                }
            }
        }

        alias_def
    }
}
