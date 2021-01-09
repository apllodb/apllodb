pub(crate) mod transaction_id;

use crate::{ApllodbResult, DatabaseName};
use std::{borrow::Borrow, fmt::Debug};

use self::transaction_id::TransactionId;

use super::database::Database;

/// Transaction interface.
///
/// It has methods to control transaction's lifetime (BEGIN, COMMIT/ABORT)
pub trait Transaction: Debug + Sized {
    /// Database who begins this transaction.
    type Db: Database;

    /// Database's ownership or reference to generate a transaction.
    type RefDb: Borrow<Self::Db>;

    /// Transaction ID.
    type TID: TransactionId;

    /// Transaction ID
    fn id(&self) -> &Self::TID;

    /// Begins a transaction.
    fn begin(db: Self::RefDb) -> ApllodbResult<Self>;

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
