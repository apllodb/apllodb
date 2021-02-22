use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{AliasName, ColumnName, CorrelationReference, TableName, TableWithAlias};

use super::field_reference::FieldReference;

/// Full field reference is in a "(correlation.)?field" form.
///
/// It has a *resolved* correlation for a field name with omitted correlation.
/// E.g. `SELECT c FROM t  -- t is omitted` -> `t.c`
///
/// In some cases, correlation does not exist.
/// E.g. `SELECT 1 AS a` -> `a`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct FullFieldReference {
    correlation_reference: Option<CorrelationReference>,
    field_reference: FieldReference,
}

impl Display for FullFieldReference {
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

impl FullFieldReference {
    pub(in crate::data_structure::reference) fn new(
        correlation_reference: Option<CorrelationReference>,
        field_reference: FieldReference,
    ) -> Self {
        Self {
            correlation_reference,
            field_reference,
        }
    }

    /// Constructor for INSERT/SELECT
    pub fn new_for_modification(table_name: TableName, column_name: ColumnName) -> Self {
        Self::new(
            Some(CorrelationReference::from(TableWithAlias {
                table_name,
                alias: None,
            })),
            FieldReference::from(column_name),
        )
    }

    /// Get ref of CorrelationReference
    pub fn as_correlation_reference(&self) -> Option<&CorrelationReference> {
        self.correlation_reference.as_ref()
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
