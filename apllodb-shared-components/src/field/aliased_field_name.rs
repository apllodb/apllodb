use serde::{Deserialize, Serialize};

use crate::{
    record_index::named_record_index::NamedRecordIndex, record_schema::RecordSchema, FieldAlias,
    FieldName, SchemaName,
};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedFieldName {
    pub(crate) field_name: FieldName,
    pub(crate) field_alias: Option<FieldAlias>,
}

impl SchemaName<RecordSchema> for AliasedFieldName {
    fn matches(&self, index: &NamedRecordIndex) -> bool {
        todo!()
    }
}

impl From<&AliasedFieldName> for NamedRecordIndex {
    fn from(n: &AliasedFieldName) -> Self {
        NamedRecordIndex::from(&n.field_name)
    }
}
