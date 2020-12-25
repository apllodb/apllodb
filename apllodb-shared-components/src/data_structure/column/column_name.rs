use crate::{data_structure::validation_helper::short_name::ShortName, error::ApllodbResult};
use serde::{Deserialize, Serialize};

/// Column name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnName(ShortName);

impl ColumnName {
    /// Constructor.
    ///
    /// # Failures
    /// - [NameTooLong](crate::ApllodbErrorKind::NameTooLong) when:
    ///   - `name` length is longer than 64 (counted as UTF-8 character).
    pub fn new<S: Into<String>>(name: S) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// Ref to column name
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
