use std::fmt::Debug;

use apllodb_shared_components::{ApllodbResult, SessionWithDb, TransactionId};

/// Transaction access methods interface.
pub trait TransactionMethods: Debug {
    /// Begins a transaction.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has already begun in this session.
    fn begin(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        let tid = self.begin_core(session)?;
        session.set_tid(tid)
    }

    #[doc(hidden)]
    fn begin_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<TransactionId>;

    /// Commit a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    fn commit(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        self.commit_core(session)?;
        session.unset_tx()
    }

    #[doc(hidden)]
    fn commit_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    fn abort(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        self.abort_core(session)?;
        session.unset_tx()
    }

    #[doc(hidden)]
    fn abort_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()>;
}
