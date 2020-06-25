use apllodb_shared_components::data_structure::ColumnName;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Primary Key.
pub trait PrimaryKey: Eq + PartialEq + Hash + Debug + Serialize + DeserializeOwned {
    /// PK's column names (more than 1 columns for compound PK).
    fn column_names(&self) -> &[ColumnName];
}
