use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Primary Key.
pub trait PrimaryKey: Eq + PartialEq + Hash + Debug + Serialize + DeserializeOwned {}
