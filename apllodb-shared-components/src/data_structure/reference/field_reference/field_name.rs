use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{data_structure::validation_helper::short_name::ShortName, ApllodbResult, ColumnName};

/// Name of a field.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct FieldName(ShortName);

impl Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl From<ColumnName> for FieldName {
    fn from(column_name: ColumnName) -> Self {
        Self::new(column_name.as_str()).expect("both ShortName")
    }
}

impl FieldName {
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
