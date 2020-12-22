use crate::{data_structure::validation_helper::short_name::ShortName, error::ApllodbResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Table name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TableName(ShortName);

impl Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TableName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](error/enum.ApllodbErrorKind.html#variant.NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
