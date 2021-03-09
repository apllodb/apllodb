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
    /// get ref to TableName
    pub fn as_table_name(&self) -> &TableName {
        match self {
            CorrelationReference::TableNameVariant(tn) => tn,
            CorrelationReference::TableAliasVariant { table_name, .. } => table_name,
        }
    }
}
