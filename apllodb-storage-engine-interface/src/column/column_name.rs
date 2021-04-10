use apllodb_shared_components::{ApllodbResult, SchemaIndex, ShortName};
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
    pub fn new(name: impl ToString) -> ApllodbResult<Self> {
        let sn = ShortName::new(name)?;
        Ok(Self(sn))
    }

    /// Ref to column name
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Whether the index hits to this column name
    pub fn matches(&self, index: &SchemaIndex) -> bool {
        index.attr() == self.as_str()
    }
}
