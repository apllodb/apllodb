use serde::{Deserialize, Serialize};

use crate::{record_index::named_record_index::NamedRecordIndex, FieldAlias, FieldName};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedFieldName {
    name: FieldName,
    alias: Option<FieldAlias>,
}

impl AliasedFieldName {
    pub(crate) fn matches(&self, _named_idx: &NamedRecordIndex) -> bool {
        todo!()
    }
}
