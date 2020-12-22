use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, hash::Hash};

pub trait Entity {
    type Id: Eq + PartialEq + Ord + PartialOrd + Hash + Debug + Serialize + DeserializeOwned;

    fn id(&self) -> &Self::Id;
}
