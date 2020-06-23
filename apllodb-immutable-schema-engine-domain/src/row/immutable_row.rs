use apllodb_storage_engine_interface::Row;

/// Immutable row who is never updated or deleted by any transaction.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ImmutableRow {
    row: Row,

    pk: TruePK,

    // Used for time-machine (TODO naming...) feature.
    tx_id: TransactionId,
}
