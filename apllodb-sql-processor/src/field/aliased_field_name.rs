use apllodb_shared_components::{SchemaIndex, SchemaName};
use apllodb_storage_engine_interface::TableColumnName;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::attribute_name::AttributeName,
    correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_name::CorrelationName,
    },
};

use super::{field_alias::FieldAlias, field_name::FieldName};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedFieldName {
    pub(crate) field_name: FieldName,
    pub(crate) field_alias: Option<FieldAlias>,
}

impl SchemaName for AliasedFieldName {
    fn _attr_matches(&self, attr: &str) -> bool {
        self.field_name.attribute_name.as_str() == attr
            || self
                .field_alias
                .as_ref()
                .map_or_else(|| false, |alias| alias.as_str() == attr)
    }

    fn _prefix_matches(&self, prefix: &str) -> bool {
        let aliased_correlation_name = &self.field_name.aliased_correlation_name;
        aliased_correlation_name.correlation_name.as_str() == prefix
            || aliased_correlation_name
                .correlation_alias
                .as_ref()
                .map_or_else(|| false, |alias| alias.as_str() == prefix)
    }
}

impl AliasedFieldName {
    pub(crate) fn to_table_column_name(&self) -> TableColumnName {
        let table_name = match &self.field_name.aliased_correlation_name.correlation_name {
            CorrelationName::TableNameVariant(table_name) => table_name.clone(),
        };
        let column_name = match &self.field_name.attribute_name {
            AttributeName::ColumnNameVariant(column_name) => column_name.clone(),
        };
        TableColumnName::new(table_name, column_name)
    }

    pub(crate) fn as_attribute_name(&self) -> &AttributeName {
        &self.field_name.attribute_name
    }
}

impl From<&AliasedFieldName> for SchemaIndex {
    fn from(n: &AliasedFieldName) -> Self {
        SchemaIndex::from(&n.field_name)
    }
}

impl From<&TableColumnName> for AliasedFieldName {
    fn from(tc: &TableColumnName) -> Self {
        let correlation_name = CorrelationName::TableNameVariant(tc.as_table_name().clone());
        let aliased_correlation_name = AliasedCorrelationName::new(correlation_name, None);

        let attribute_name = AttributeName::ColumnNameVariant(tc.as_column_name().clone());

        let field_name = FieldName::new(aliased_correlation_name, attribute_name);

        Self::new(field_name, None)
    }
}
