use super::{column::ColumnDataType, Version, VersionNumber};
use serde::{Deserialize, Serialize};

/// Inactive Version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct InactiveVersion(Version);

impl InactiveVersion {
    /// Version number.
    pub fn number(&self) -> &VersionNumber {
        &self.0.number
    }

    /// Ref to columns and their data types.
    pub fn column_data_types(&self) -> &[ColumnDataType] {
        &self.0.column_data_types
    }
}
