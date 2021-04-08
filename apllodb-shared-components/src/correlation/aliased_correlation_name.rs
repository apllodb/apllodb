use crate::{CorrelationAlias, CorrelationName};
use serde::{Deserialize, Serialize};

/// An alias to a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct AliasedCorrelationName {
    name: CorrelationName,
    alias: Option<CorrelationAlias>,
}
