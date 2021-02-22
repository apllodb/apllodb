pub(crate) mod correlation_name;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{traits::correlation::Correlation, AliasName, TableWithAlias};

use self::correlation_name::CorrelationName;

/// Name & alias of a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct CorrelationReference {
    name: CorrelationName,
    alias: Option<AliasName>,
}

impl Display for CorrelationReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.as_str())
    }
}

impl From<TableWithAlias> for CorrelationReference {
    fn from(table_with_alias: TableWithAlias) -> Self {
        Self {
            name: CorrelationName::from(table_with_alias.table_name),
            alias: table_with_alias.alias,
        }
    }
}

impl Correlation for CorrelationReference {
    fn is_named(&self, correlation_name: &CorrelationName) -> bool {
        &self.name == correlation_name
            || self.alias.as_ref().map_or_else(
                || false,
                |alias| alias.as_str() == correlation_name.as_str(),
            )
    }
}

impl CorrelationReference {
    /// as CorrelationName
    pub fn as_correlation_name(&self) -> &CorrelationName {
        &self.name
    }

    /// as AliasName
    pub fn as_alias(&self) -> Option<&AliasName> {
        self.alias.as_ref()
    }
}
