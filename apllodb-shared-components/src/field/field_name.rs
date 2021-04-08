use serde::{Deserialize, Serialize};

use crate::{correlation::aliased_correlation_name::AliasedCorrelationName, AttributeName};

/// Name of a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct FieldName {
    correlation: AliasedCorrelationName,
    attribute: AttributeName,
}
