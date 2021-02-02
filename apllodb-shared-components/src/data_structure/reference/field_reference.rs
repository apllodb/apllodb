use std::fmt::Display;

use crate::{AliasName, ColumnName};
use serde::{Deserialize, Serialize};

/// Reference to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum FieldReference {
    /// column name
    ColumnNameVariant(ColumnName),
    /// alias to column
    ColumnAliasVariant {
        /// alias
        alias_name: AliasName,
        /// reffed
        column_name: ColumnName,
    },
    // TODO FieldAliasVariant { ... }
}

impl Display for FieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FieldReference::ColumnNameVariant(cn) => cn.as_str(),
            FieldReference::ColumnAliasVariant { alias_name, .. } => alias_name.as_str(),
        };
        write!(f, "{}", s)
    }
}
