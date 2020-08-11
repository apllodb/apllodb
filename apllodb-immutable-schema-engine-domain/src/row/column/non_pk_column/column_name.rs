use apllodb_shared_components::{data_structure::ColumnName, error::ApllodbResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct NonPKColumnName(ColumnName);

impl From<ColumnName> for NonPKColumnName {
    fn from(cn: ColumnName) -> Self {
        Self(cn)
    }
}

impl Display for NonPKColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl NonPKColumnName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](error/enum.ApllodbErrorKind.html#variant.NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let cn = ColumnName::new(name)?;
        Ok(Self(cn))
    }

    /// Ref to column name
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Ref to ColumnName
    pub fn as_column_name(&self) -> &ColumnName {
        &self.0
    }
}
