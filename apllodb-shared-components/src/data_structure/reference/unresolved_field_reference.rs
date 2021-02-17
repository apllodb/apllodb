use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::CorrelationReference, AliasName,
    ApllodbResult, ColumnName, FromItem, FullFieldReference, TableName,
};

use super::{field_reference::FieldReference, FieldReferenceBase};

/// Unresolved field reference is in a "(correlation.)?field" form.
///
/// It's correlation may be omitted in SQL.
/// E.g. `SELECT c FROM t  -- t is omitted`
///
/// Omitted correlation can be *resolved* by FromItem.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct UnresolvedFieldReference(FieldReferenceBase);

impl Display for UnresolvedFieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl UnresolvedFieldReference {
    /// Constructor
    pub fn new(
        correlation_reference: Option<CorrelationReference>,
        field_reference: FieldReference,
    ) -> Self {
        let base = FieldReferenceBase::new(correlation_reference, field_reference);
        Self(base)
    }

    /// into FullFieldReference
    pub fn resolve(self, _ast_from_items: Vec<FromItem>) -> ApllodbResult<FullFieldReference> {
        todo!()
    }

    /// Get ref of CorrelationReference
    pub fn as_correlation_reference(&self) -> Option<&CorrelationReference> {
        self.0.as_correlation_reference()
    }

    /// Get ref of TableName
    pub fn as_table_name(&self) -> Option<&TableName> {
        self.0.as_table_name()
    }

    /// Get ref of FieldReference
    pub fn as_field_reference(&self) -> &FieldReference {
        self.0.as_field_reference()
    }

    /// Get ref of ColumnName
    pub fn as_column_name(&self) -> &ColumnName {
        self.0.as_column_name()
    }

    /// Set correlation reference
    ///
    /// # Panics
    ///
    /// When correlation does not exist.
    pub fn set_correlation_alias(&mut self, correlation_alias: AliasName) {
        self.0.set_correlation_alias(correlation_alias)
    }

    /// Set field reference
    pub fn set_field_alias(&mut self, field_alias: AliasName) {
        self.0.set_field_alias(field_alias)
    }
}
