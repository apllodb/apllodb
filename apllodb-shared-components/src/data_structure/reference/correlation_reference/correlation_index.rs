use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::CorrelationReference;

/// Matcher to [CorrelationReference](crate::CorrelationReference).
///
/// # Panics
///
/// When constructed from invalid-formed string.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct CorrelationIndex {
    correlation_name: String,
}

impl CorrelationIndex {
    fn matches(&self, correlation_reference: &CorrelationReference) -> bool {
        match correlation_reference {
            CorrelationReference::TableNameVariant(table_name) => {
                self.correlation_name.as_str() == table_name.as_str()
            }
            CorrelationReference::TableAliasVariant {
                alias_name,
                table_name,
            } => {
                self.correlation_name.as_str() == table_name.as_str()
                    || self.correlation_name.as_str() == alias_name.as_str()
            }
        }
    }
}

impl Display for CorrelationIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.correlation_name)
    }
}

impl<S: Into<String>> From<S> for CorrelationIndex {
    fn from(s: S) -> Self {
        Self {
            correlation_name: s.into(),
        }
    }
}

impl From<CorrelationReference> for CorrelationIndex {
    fn from(correlation_reference: CorrelationReference) -> Self {
        match correlation_reference {
            CorrelationReference::TableNameVariant(table_name)
            | CorrelationReference::TableAliasVariant { table_name, .. } => {
                Self::from(table_name.as_str())
            }
        }
    }
}
