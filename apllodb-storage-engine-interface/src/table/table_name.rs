use apllodb_shared_components::{ApllodbResult, ShortName};
use serde::{Deserialize, Serialize};

/// Table name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TableName(ShortName);

impl TableName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](apllodb_shared_components::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new(name: impl ToString) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// Table name as str
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
