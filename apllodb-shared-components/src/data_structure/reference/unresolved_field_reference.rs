use std::fmt::Display;

use apllodb_sql_parser::apllodb_ast;
use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::CorrelationReference, AliasName,
    ApllodbResult, ColumnName, FullFieldReference,
};

use super::field_reference::FieldReference;

/// Unresolved field reference is in a "(correlation.)?field" form.
///
/// It's correlation may be omitted in SQL.
/// E.g. `SELECT c FROM t  -- t is omitted`
///
/// Omitted correlation can be *resolved* by FromItem.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct UnresolvedFieldReference {
    correlation_reference: Option<CorrelationReference>,
    field_reference: FieldReference,
}

impl Display for UnresolvedFieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(corr) = self.correlation_reference.as_ref() {
                format!("{}.", corr)
            } else {
                "".to_string()
            },
            self.field_reference
        )
    }
}

impl UnresolvedFieldReference {
    /// into FullFieldReference
    /// TODO use domain FromItem
    pub fn resolve(
        self,
        _ast_from_items: Vec<apllodb_ast::FromItem>,
    ) -> ApllodbResult<FullFieldReference> {
        todo!()
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
