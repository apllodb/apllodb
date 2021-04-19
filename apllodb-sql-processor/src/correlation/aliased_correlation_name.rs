use serde::{Deserialize, Serialize};

use super::{correlation_alias::CorrelationAlias, correlation_name::CorrelationName};

/// Correlation name with/without an alias.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedCorrelationName {
    pub(crate) correlation_name: CorrelationName,
    pub(crate) correlation_alias: Option<CorrelationAlias>,
}
