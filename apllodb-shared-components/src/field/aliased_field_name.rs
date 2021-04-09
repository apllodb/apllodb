use serde::{Deserialize, Serialize};

use crate::{FieldAlias, FieldName, SchemaIndex, SchemaName};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedFieldName {
    pub(crate) field_name: FieldName,
    pub(crate) field_alias: Option<FieldAlias>,
}

impl SchemaName for AliasedFieldName {
    fn matches(&self, index: &SchemaIndex) -> bool {
        todo!()
    }
}

impl From<&AliasedFieldName> for SchemaIndex {
    fn from(n: &AliasedFieldName) -> Self {
        SchemaIndex::from(&n.field_name)
    }
}
