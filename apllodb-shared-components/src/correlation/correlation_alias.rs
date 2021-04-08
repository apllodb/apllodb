use crate::{validation_helper::short_name::ShortName, ApllodbResult};
use serde::{Deserialize, Serialize};

/// An alias to a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct CorrelationAlias(ShortName);

impl CorrelationAlias {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](crate::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: ToString>(name: S) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// Alias name as str
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
