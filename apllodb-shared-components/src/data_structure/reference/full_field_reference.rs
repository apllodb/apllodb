use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::CorrelationReference, AliasName, ColumnName,
    TableName,
};

use super::field_reference::FieldReference;

/// Full field reference is in a "(correlation.)?field" form.
///
/// It has a *resolved* correlation for a field name with omitted correlation.
/// E.g. `SELECT c FROM t  -- t is omitted` -> `t.c`
///
/// In some cases, correlation does not exist.
/// E.g. `SELECT 1 AS a` -> `a`
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
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
    /// Get ref of CorrelationReference
    pub fn as_correlation_reference(&self) -> Option<&CorrelationReference> {
        self.correlation_reference.as_ref()
    }

    /// Get ref of TableName
    pub fn as_table_name(&self) -> Option<&TableName> {
        self.as_correlation_reference().map(|corr| match corr {
            CorrelationReference::TableNameVariant(tn) => tn,
            CorrelationReference::TableAliasVariant { table_name, .. } => table_name,
        })
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

    /// Set correlation reference
    ///
    /// # Panics
    ///
    /// When correlation does not exist.
    pub fn set_correlation_alias(&mut self, correlation_alias: AliasName) {
        let cur_table_name = self.as_table_name().expect("correlation does not exist");
        self.correlation_reference = Some(CorrelationReference::TableAliasVariant {
            alias_name: correlation_alias,
            table_name: cur_table_name.clone(),
        });
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
