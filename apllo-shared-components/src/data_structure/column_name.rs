use crate::error::{AplloError, AplloErrorKind, AplloResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Column name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(String);

impl ColumnName {
    /// Constructor.
    ///
    /// Asserting that `name` contains only valid characters and is not the same as SQL keyword
    /// (since it should be assured by tokenizer).
    ///
    /// # Failures
    /// - [NameTooLong](error/enum.AplloErrorKind.html#variant.NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new(name: String) -> AplloResult<Self> {
        if (name.len() > 64) {
            Err(AplloError::new(
                AplloErrorKind::NameTooLong,
                format!("ColumnName `{}` is too long ({} > 64)", name, name.len()),
                None,
            ))
        } else {
            Ok(Self(name))
        }
    }
}

impl Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
