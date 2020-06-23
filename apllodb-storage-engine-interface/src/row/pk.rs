use apllodb_shared_components::data_structure::ColumnName;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Primary Key.
pub trait PrimaryKey: Eq + PartialEq + Hash + Debug + Serialize + DeserializeOwned {
    /// PK's column name.
    fn column_name(&self) -> &ColumnName;
}
