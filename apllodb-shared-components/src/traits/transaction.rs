pub(crate) mod transaction_id;

use crate::{ApllodbResult, DatabaseName};
use std::fmt::Debug;

use self::transaction_id::TransactionId;

use super::database::Database;

/// Transaction interface.
///
/// It has methods to control transaction's lifetime (BEGIN, COMMIT/ABORT)
/// and storage engine's access methods (like system calls in OS).
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
///
/// Implementation of this trait can either execute physical transaction operations (e.g. locking objects, writing logs to disk, etc...)
/// directly or delegate physical operations to another object.
pub trait Transaction: Debug + Sized {
    /// Database's ownership or reference to generate a transaction.
    type Db: Database;

    /// Transaction ID.
    type TID: TransactionId;

    /// Transaction ID
    fn id(&self) -> &Self::TID;

    /// Begins a transaction.
    ///
    /// Note that this function takes reference to database in order for a database to begin multiple transactions.
    /// Implementer may need interior mutability inside Database implementation.
    fn begin(db: &Self::Db) -> ApllodbResult<Self>;

    /// Commit a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    fn commit(self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(self) -> ApllodbResult<()>;

    /// Ref to database name.
    fn database_name(&self) -> &DatabaseName;
}
