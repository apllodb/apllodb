use crate::{error::ApllodbResult, validation_helper::short_name::ShortName};
use serde::{Deserialize, Serialize};

/// Database name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct DatabaseName(ShortName);

impl DatabaseName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](crate::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// database name
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
