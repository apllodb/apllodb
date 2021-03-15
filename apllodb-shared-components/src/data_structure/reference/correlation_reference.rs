pub(crate) mod correlation_index;

use std::fmt::Display;

use crate::{AliasName, TableName};
use serde::{Deserialize, Serialize};

/// Reference to a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CorrelationReference {
    /// table name
    TableNameVariant(TableName),

    /// alias to table
    TableAliasVariant {
        /// alias
        alias_name: AliasName,
        /// reffed
        table_name: TableName,
    },
    // TODO SubQueryAliasVariant { ... }
}

impl Display for CorrelationReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CorrelationReference::TableNameVariant(tn) => tn.as_str(),
            CorrelationReference::TableAliasVariant { alias_name, .. } => alias_name.as_str(),
        };
        write!(f, "{}", s)
    }
}

impl CorrelationReference {
    /// check if `correlation_name` matches to this column reference
    pub fn matches(&self, correlation_name: &str) -> bool {
        match self {
            CorrelationReference::TableNameVariant(tn) => tn.as_str() == correlation_name,
            CorrelationReference::TableAliasVariant {
                alias_name,
                table_name,
            } => alias_name.as_str() == correlation_name || table_name.as_str() == correlation_name,
        }
    }

    /// get ref to TableName
    pub fn as_table_name(&self) -> &TableName {
        match self {
            CorrelationReference::TableNameVariant(tn) => tn,
            CorrelationReference::TableAliasVariant { table_name, .. } => table_name,
        }
    }
}
