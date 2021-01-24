use crate::error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult};
use serde::{Deserialize, Serialize};

/// Short (64 chars in UTF-8 at maximum) object name used for table names, column names, and so on.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::data_structure) struct ShortName(String);

impl ShortName {
    /// Constructor.
    ///
    /// Asserting that `name` contains only valid characters and is not the same as SQL keyword
    /// (since it should be assured by tokenizer).
    ///
    /// # Failures
    /// - [NameTooLong](apllodb-shared-components::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub(in crate::data_structure) fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let name = name.into();
        Self::validate_length(&name)?;
        Ok(Self(name))
    }

    pub(in crate::data_structure) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn validate_length(name: &str) -> ApllodbResult<()> {
        if name.chars().count() > 64 {
            Err(ApllodbError::new(
                ApllodbErrorKind::NameTooLong,
                format!(
                    "ShortName `{}` is too long ({} > 64)",
                    name,
                    name.chars().count()
                ),
                None,
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ShortName;
    use crate::{error::kind::ApllodbErrorKind, test_support::setup};

    #[test]
    fn test_success() {
        setup();

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
        setup();

        let names = vec!["a".repeat(65), "あ".repeat(65), "💪".repeat(65)];

        for name in &names {
            match ShortName::new(name) {
                Err(e) => match e.kind() {
                    ApllodbErrorKind::NameTooLong => {}
                    x => panic!("{} : unexpected error: {:?}", name, x),
                },
                Ok(_) => panic!("{} : should be an error", name),
            }
        }
    }
}
