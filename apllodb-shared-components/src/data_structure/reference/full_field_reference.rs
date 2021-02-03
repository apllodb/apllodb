use std::convert::TryFrom;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::CorrelationReference, AliasName,
    ApllodbError, ApllodbResult, ColumnName, FieldIndex, TableName,
};

use super::field_reference::FieldReference;

/// Full field reference == "correlation.field".
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct FullFieldReference {
    correlation_reference: CorrelationReference,
    field_reference: FieldReference,
}

impl Display for FullFieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.correlation_reference, self.field_reference)
    }
}

impl TryFrom<FieldIndex> for FullFieldReference {
    type Error = ApllodbError;

    /// # Panics
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - this field index does not represent a valid field.
    fn try_from(field: FieldIndex) -> ApllodbResult<Self> {
        match field {
            FieldIndex::InFullFieldReference(ffr) => Ok(ffr),
        }
    }
}

impl FullFieldReference {
    /// Get ref of TableName
    pub fn as_table_name(&self) -> &TableName {
        match &self.correlation_reference {
            CorrelationReference::TableNameVariant(tn) => tn,
            CorrelationReference::TableAliasVariant { table_name, .. } => table_name,
        }
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
    pub fn set_correlation_alias(&mut self, correlation_alias: AliasName) {
        let cur_table_name = self.as_table_name();
        self.correlation_reference = CorrelationReference::TableAliasVariant {
            alias_name: correlation_alias,
            table_name: cur_table_name.clone(),
        };
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
