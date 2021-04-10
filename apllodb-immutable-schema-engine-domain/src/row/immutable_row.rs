pub mod builder;

use apllodb_storage_engine_interface::Row;
use serde::{Deserialize, Serialize};

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ImmutableRow {
    pub row: Row,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}
