use serde::{Deserialize, Serialize};

use crate::{
    correlation::aliased_correlation_name::AliasedCorrelationName,
    record_index::named_record_index::NamedRecordIndex, AttributeName,
};

/// Name of a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct FieldName {
    correlation: AliasedCorrelationName,
    attribute: AttributeName,
}

impl From<&FieldName> for NamedRecordIndex {
    fn from(n: &FieldName) -> Self {
        let s = format!("{}.{}", n.correlation.name(), n.attribute);
        NamedRecordIndex::from(s.as_str())
    }
}
