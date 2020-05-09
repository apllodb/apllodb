use crate::error::{AplloError, AplloErrorKind, AplloResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::data_structure) struct ShortName(String);

impl ShortName {
    /// Constructor.
    ///
    /// Asserting that `name` contains only valid characters and is not the same as SQL keyword
    /// (since it should be assured by tokenizer).
    ///
    /// # Failures
    /// - [NameTooLong](error/enum.AplloErrorKind.html#variant.NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub(in crate::data_structure) fn new<S: Into<String>>(name: S) -> AplloResult<Self> {
        let name = name.into();

        if name.chars().count() > 64 {
            Err(AplloError::new(
                AplloErrorKind::NameTooLong,
                format!(
                    "ShortName `{}` is too long ({} > 64)",
                    name,
                    name.chars().count()
                ),
                None,
            ))
        } else {
            Ok(Self(name))
        }
    }
}

impl Display for ShortName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::ShortName;
    use crate::error::AplloErrorKind;

    #[test]
    fn test_success() {
        let names = vec!["a".repeat(64), "あ".repeat(64), "💪".repeat(64)];

        for name in &names {
            match ShortName::new(name) {
                Ok(_) => {}
                Err(e) => panic!("{} : unexpected error: {:?}", name, e),
            }
        }
    }

    #[test]
    fn test_failure_too_long_name() {
        let names = vec!["a".repeat(65), "あ".repeat(65), "💪".repeat(65)];

        for name in &names {
            match ShortName::new(name) {
                Err(e) => match e.kind() {
                    AplloErrorKind::NameTooLong => {}
                    x => panic!("{} : unexpected error: {:?}", name, x),
                },
                Ok(_) => panic!("{} : should be an error", name),
            }
        }
    }
}
