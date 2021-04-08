use serde::{Deserialize, Serialize};

use crate::{FieldAlias, FieldName};

/// An alias to a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct AliasedFieldName {
    name: FieldName,
    alias: Option<FieldAlias>,
}
