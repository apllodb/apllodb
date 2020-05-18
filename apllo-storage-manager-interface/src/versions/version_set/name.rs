use apllo_shared_components::data_structure::{ShortName, TableName};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VersionSetName(ShortName);

impl From<TableName> for VersionSetName {
    fn from(table_name: TableName) -> Self {
        Self(table_name.as_short_name().clone())
    }
}
