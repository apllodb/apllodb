use crate::DbCtxLike;
use apllodb_shared_components::error::ApllodbResult;

/// Transaction context interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait TxCtxLike {
    /// Database context shared among many transactions.
    type DbCtx: DbCtxLike;

    /// Start a transaction.
    fn begin(db: &Self::DbCtx) -> ApllodbResult<Self>
    where
        Self: std::marker::Sized;

    /// Commit a transaction.
    fn commit(&mut self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(&mut self) -> ApllodbResult<()>;
}
