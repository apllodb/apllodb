use crate::field::aliased_field_name::AliasedFieldName;
use apllodb_storage_engine_interface::TableColumnName;
use serde::{Deserialize, Serialize};

/// Aliaser is a glue of names between Storage Engines and SQL Processor.
///
/// Storage Engines use TableColumnName as a full column name, while SQL Processor uses AliasedFieldName as a fully-aliased field name.
/// Rows from Storage Engines do not contain aliases but Records used everywhere in SQL Processor should hold them in order for SchemaIndex to correctly pick aliased field names.
///
/// Internally, Aliaser is just a sequence of AliasedFieldNames because an AliasedFieldName has natural conversion into TableColumnName.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct Aliaser(Vec<AliasedFieldName>);

impl Aliaser {
    pub(crate) fn alias(&self, table_column_name: &TableColumnName) -> AliasedFieldName {
        self.0
            .iter()
            .find_map(|afn| {
                if &afn.to_table_column_name() == table_column_name {
                    Some(afn.clone())
                } else {
                    None
                }
            })
            .unwrap_or(AliasedFieldName::from(table_column_name))
    }
}

impl From<Vec<AliasedFieldName>> for Aliaser {
    fn from(aliased_field_names: Vec<AliasedFieldName>) -> Self {
        Self(aliased_field_names)
    }
}
