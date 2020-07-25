use apllodb_shared_components::data_structure::ColumnDefinition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct NonPKColumnDefinition(pub ColumnDefinition);
