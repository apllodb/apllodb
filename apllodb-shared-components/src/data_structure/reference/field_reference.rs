pub(crate) mod field_name;

use std::fmt::Display;

use crate::{AliasName, ColumnName, FieldName};
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

impl From<ColumnName> for FieldReference {
    fn from(column_name: ColumnName) -> Self {
        Self::ColumnNameVariant(column_name)
    }
}

impl FieldReference {
    /// as column name
    pub fn as_column_name(&self) -> &ColumnName {
        match self {
            FieldReference::ColumnNameVariant(column_name)
            | FieldReference::ColumnAliasVariant { column_name, .. } => column_name,
        }
    }

    /// as field alias
    pub fn as_alias_name(&self) -> Option<&AliasName> {
        match self {
            FieldReference::ColumnNameVariant(_) => None,
            FieldReference::ColumnAliasVariant { alias_name, .. } => Some(alias_name),
        }
    }

    /// compares a name with field name or alias
    pub fn is_named(&self, field_name: &FieldName) -> bool {
        match self {
            FieldReference::ColumnNameVariant(column_name) => {
                column_name.as_str() == field_name.as_str()
            }
            FieldReference::ColumnAliasVariant {
                alias_name,
                column_name,
            } => {
                column_name.as_str() == field_name.as_str()
                    || alias_name.as_str() == field_name.as_str()
            }
        }
    }
}
