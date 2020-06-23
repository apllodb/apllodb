use std::{fmt::Debug, hash::Hash};

/// Transaction ID.
pub trait TransactionId: Eq + PartialEq + Ord + PartialOrd + Hash + Debug {}
