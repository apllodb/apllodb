use apllodb_shared_components::error::ApllodbResult;
use std::fmt::Debug;

/// Transaction context interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait TxCtxLike: Eq + PartialEq + Debug {
    /// Commit a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    fn commit(self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(self) -> ApllodbResult<()>;
}
