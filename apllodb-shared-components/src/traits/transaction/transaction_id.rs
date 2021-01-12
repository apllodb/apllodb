use std::{fmt::Debug, hash::Hash};

/// Transaction ID.
///
/// It is a storage engine's responsibility to generate underlying u64 ID.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TransactionId(i64);

impl TransactionId {
    /// Generate transaction ID.
    pub fn new(id: i64) {
        Self(id)
    }
}
