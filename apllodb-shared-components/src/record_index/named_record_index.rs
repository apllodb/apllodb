use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

impl Display for NamedRecordIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre = if let Some(corr) = &self.correlation {
            format!("{}.", corr)
        } else {
            "".to_string()
        };
        write!(f, "{}{}", pre, self.attribute)
    }
}

impl From<&str> for NamedRecordIndex {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('.').collect();

        debug_assert!(!parts.is_empty());
        assert!(parts.len() <= 2, "too many dots (.) !");

        parts.iter().for_each(|part| {
            assert!(
                !part.is_empty(),
                "correlation name nor field name must not be empty string"
            )
        });

        let first = parts
            .get(0)
            .expect("must have at least 1 part")
            .trim()
            .to_string();
        let second = parts.get(1).map(|s| s.trim().to_string());

        if let Some(second) = second {
            Self {
                correlation: Some(first),
                attribute: second,
            }
        } else {
            Self {
                correlation: None,
                attribute: first,
            }
        }
    }
}
