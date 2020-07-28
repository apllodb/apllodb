use apllodb_shared_components::{error::ApllodbResult, data_structure::ColumnName};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// ApparentPrimaryKey without values.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct PKColumnName(ColumnName);

impl From<ColumnName> for PKColumnName {
    fn from(cn: ColumnName) -> Self {
        Self(cn)
    }
}

impl Display for PKColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PKColumnName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](error/enum.ApllodbErrorKind.html#variant.NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let cn = ColumnName::new(name)?;
        Ok(Self(cn))
    }

    pub fn as_column_name(&self) -> &ColumnName {
        &self.0
    }

    /// Ref to column name
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
