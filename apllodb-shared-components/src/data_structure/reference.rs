use std::fmt::Display;

use crate::{AliasName, ColumnName, CorrelationName, FieldReference};
use serde::{Deserialize, Serialize};

pub(crate) mod correlation_name;
pub(crate) mod field_reference;
pub(crate) mod full_field_reference;
pub(crate) mod select_field_reference;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
struct FieldReferenceBase {
    correlation_name: Option<CorrelationName>,
    field_reference: FieldReference,
}

impl Display for FieldReferenceBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(corr) = self.as_correlation_reference() {
                format!("{}.", corr)
            } else {
                "".to_string()
            },
            self.field_reference
        )
    }
}

impl FieldReferenceBase {
    /// Get ref of CorrelationReference
    pub fn as_correlation_reference(&self) -> Option<&CorrelationName> {
        self.correlation_name.as_ref()
    }

    /// Get ref of FieldReference
    pub fn as_field_reference(&self) -> &FieldReference {
        &self.field_reference
    }

    /// Get ref of ColumnName
    pub fn as_column_name(&self) -> &ColumnName {
        match &self.field_reference {
            FieldReference::ColumnNameVariant(cn) => cn,
            FieldReference::ColumnAliasVariant { column_name, .. } => column_name,
        }
    }

    /// Set field reference
    pub fn set_field_alias(&mut self, field_alias: AliasName) {
        let cur_column_name = self.as_column_name();
        self.field_reference = FieldReference::ColumnAliasVariant {
            alias_name: field_alias,
            column_name: cur_column_name.clone(),
        };
    }
}
