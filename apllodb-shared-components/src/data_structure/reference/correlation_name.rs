use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{data_structure::validation_helper::short_name::ShortName, ApllodbResult, TableName};

/// Name of a correlation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct CorrelationName(ShortName);

impl Display for CorrelationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl From<TableName> for CorrelationName {
    fn from(table_name: TableName) -> Self {
        Self::new(table_name.as_str()).expect("both ShortName")
    }
}

impl CorrelationName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](crate::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// as str
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
