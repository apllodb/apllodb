use crate::{CorrelationAlias, CorrelationName};
use serde::{Deserialize, Serialize};

/// An alias to a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct AliasedCorrelationName {
    pub(crate) correlation_name: CorrelationName,
    pub(crate) correlation_alias: Option<CorrelationAlias>,
}
