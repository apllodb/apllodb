use apllodb_shared_components::error::ApllodbResult;

/// Transaction context interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait TxCtxLike {
    /// Commit a transaction.
    fn commit(self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(self) -> ApllodbResult<()>;
}
