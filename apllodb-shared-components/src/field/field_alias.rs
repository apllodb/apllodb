use crate::{validation_helper::short_name::ShortName, ApllodbResult};
use serde::{Deserialize, Serialize};

/// An alias to a Field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct FieldAlias(ShortName);

impl FieldAlias {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](crate::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn _new(name: impl ToString) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// Alias name as str
    pub fn _as_str(&self) -> &str {
        self.0.as_str()
    }
}
