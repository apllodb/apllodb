use apllodb_shared_components::error::ApllodbResult;

/// Transaction context interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait TxCtxLike<'st> {
    /// Type of the storage instance from/to which a transaction reads/writes.
    ///
    /// Transaction and storage has N:1 relationship:
    /// Many transactions read/write from/to the same storage and a transaction reads/writes to only 1 storage.
    ///
    /// Classical storages should have page-based buffer-pool in memory layer and pages are
    /// written to disk when:
    /// - transaction is committed (FORCE).
    /// - checkpoint is made (NO-FORCE).
    /// - buffer-pool is made full by another transaction (STEAL).
    ///
    /// But Storage type here is not necessarily page-based.
    ///
    /// Common implementation targets are:
    ///
    /// - Page-based buffer pool with ARIES method.
    ///   - Appropriate for disk-oriented system.
    /// - Ones specialized in in-memory transaction. Cicada (https://hyeontaek.com/papers/cicada-sigmod2017.pdf), for example.
    type Storage;

    /// Start a transaction.
    fn begin(storage: &'st Self::Storage) -> ApllodbResult<Self>
    where
        Self: Sized;

    /// Commit a transaction.
    fn commit(&mut self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(&mut self) -> ApllodbResult<()>;
}
