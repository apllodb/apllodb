use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::SchemaIndex;

/// Matcher to [AliasedFieldName](crate::AliasedFieldName).
/// Used to get a value from a record.
///
/// # Example
///
/// ```
/// # use apllodb_shared_components::NamedRecordIndex;
///
/// let _ = NamedRecordIndex::from("c");    // column name or alias name "c"
/// let _ = NamedRecordIndex::from("t.c");  // column name or alias name "c"; inside table / table alias / subquery alias named "t".
///
/// assert_eq!(NamedRecordIndex::from("c"), NamedRecordIndex::from("  c "));
/// assert_ne!(NamedRecordIndex::from("c"), NamedRecordIndex::from("C"));
/// assert_eq!(NamedRecordIndex::from("t.c"), NamedRecordIndex::from("  t  .  c "));
/// ```
///
/// # Panics
///
/// When constructed from invalid-formed string.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct NamedRecordIndex {
    correlation: Option<String>,
    attribute: String,
}

impl SchemaIndex for NamedRecordIndex {
    fn new(prefix: Option<String>, attr: String) -> Self {
        Self {
            correlation: prefix,
            attribute: attr,
        }
    }

    fn prefix(&self) -> Option<&str> {
        self.correlation.as_ref().map(|s| s.as_str())
    }

    fn attr(&self) -> &str {
        &self.attribute
    }
}

impl Display for NamedRecordIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl From<&str> for NamedRecordIndex {
    fn from(s: &str) -> Self {
        SchemaIndex::from(s)
    }
}
