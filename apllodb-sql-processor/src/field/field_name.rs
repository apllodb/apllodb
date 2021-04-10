use serde::{Deserialize, Serialize};

use crate::correlation::aliased_correlation_name::AliasedCorrelationName;

/// Name of a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct FieldName {
    pub(crate) aliased_correlation_name: AliasedCorrelationName,
    pub(crate) attribute_name: AttributeName,
}

impl From<&FieldName> for SchemaIndex {
    fn from(n: &FieldName) -> Self {
        let s = format!(
            "{}.{}",
            n.aliased_correlation_name.correlation_name, n.attribute_name
        );
        Self::from(s.as_str())
    }
}
