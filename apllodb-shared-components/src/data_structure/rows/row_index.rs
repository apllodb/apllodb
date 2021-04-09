use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Matcher to [TableColumnName](crate::TableColumnName).
/// Used to get a value from a row.
///
/// # Panics
///
/// When constructed from invalid-formed string.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct RowIndex {
    table: Option<String>,
    column: String,
}

impl Display for RowIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre = if let Some(corr) = &self.table {
            format!("{}.", corr)
        } else {
            "".to_string()
        };
        write!(f, "{}{}", pre, self.column)
    }
}

impl From<&str> for RowIndex {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('.').collect();

        debug_assert!(!parts.is_empty());
        assert!(parts.len() <= 2, "too many dots (.) !");

        parts.iter().for_each(|part| {
            assert!(
                !part.is_empty(),
                "table name nor column name must not be empty string"
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
                table: Some(first),
                column: second,
            }
        } else {
            Self {
                table: None,
                column: first,
            }
        }
    }
}
