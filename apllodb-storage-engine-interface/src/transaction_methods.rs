use std::fmt::Debug;

use apllodb_shared_components::{ApllodbResult, SessionWithDb, TransactionId};

/// Transaction access methods interface.
///
/// It has methods to control transaction's lifetime (BEGIN, COMMIT/ABORT)
pub trait TransactionMethods: Debug {
    /// Begins a transaction.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has already begun in this session.
    fn begin(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        let tid = self.begin_core(session)?;
        session.set_tid(tid)
    }

    #[doc(hidden)]
    fn begin_core(&self, session: &mut SessionWithDb) -> ApllodbResult<TransactionId>;

    /// Commit a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    fn commit(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        session.unset_tx()?;
        self.commit_core(session)
    }

    #[doc(hidden)]
    fn commit_core(&self, session: &mut SessionWithDb) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    fn abort(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        session.unset_tx()?;
        self.abort_core(session)
    }

    #[doc(hidden)]
    fn abort_core(&self, session: &mut SessionWithDb) -> ApllodbResult<()>;
}
