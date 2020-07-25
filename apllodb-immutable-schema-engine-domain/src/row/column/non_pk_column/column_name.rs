use apllodb_shared_components::data_structure::ColumnName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct NonPKColumnName(pub ColumnName);
