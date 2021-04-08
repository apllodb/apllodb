use serde::{Deserialize, Serialize};

use crate::{record_index::named_record_index::NamedRecordIndex, FieldAlias, FieldName};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedFieldName {
    pub(crate) field_name: FieldName,
    pub(crate) field_alias: Option<FieldAlias>,
}

impl AliasedFieldName {
    pub(crate) fn matches(&self, named_idx: &NamedRecordIndex) -> bool {
        todo!()
    }
}

impl From<&AliasedFieldName> for NamedRecordIndex {
    fn from(n: &AliasedFieldName) -> Self {
        NamedRecordIndex::from(&n.field_name)
    }
}
