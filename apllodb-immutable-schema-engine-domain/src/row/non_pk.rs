use apllodb_shared_components::data_structure::ColumnName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct NonPKColumnName(ColumnName);

impl NonPKColumnName {
    /// Ref to column name
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
