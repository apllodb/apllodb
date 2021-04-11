use apllodb_shared_components::{SchemaIndex, SchemaName};
use serde::{Deserialize, Serialize};

use super::{field_alias::FieldAlias, field_name::FieldName};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedFieldName {
    pub(crate) field_name: FieldName,
    pub(crate) field_alias: Option<FieldAlias>,
}

impl SchemaName for AliasedFieldName {
    fn _attr_matches(&self, attr: &str) -> bool {
        todo!()
    }

    fn _prefix_attr_match(&self, prefix: &str, attr: &str) -> bool {
        todo!()
    }
}

impl From<&AliasedFieldName> for SchemaIndex {
    fn from(n: &AliasedFieldName) -> Self {
        SchemaIndex::from(&n.field_name)
    }
}
