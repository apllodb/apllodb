use apllodb_shared_components::error::ApllodbResult;

/// Transaction context interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait TxCtxLike {
    /// Start a transaction.
    fn begin() -> ApllodbResult<Self>
    where
        Self: Sized;

    /// Commit a transaction.
    fn commit(self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(self) -> ApllodbResult<()>;
}
