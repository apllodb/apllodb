use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

/// ID of Version-Revision Resolver's entry.
pub trait VrrId:
    Clone + Eq + PartialEq + Ord + PartialOrd + Hash + Debug + Serialize + DeserializeOwned + Sized
{
}
