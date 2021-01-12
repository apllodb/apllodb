use std::{borrow::Borrow, fmt::Debug};

use apllodb_shared_components::{ApllodbResult, SessionWithDb};

/// Transaction access methods interface.
pub trait TransactionMethods: Debug {
    /// Reference to session (may take lifetime parameter to generate physical transaction struct).
    type Sess: Borrow<SessionWithDb>;

    /// xx
    type Slf: Borrow<Self>;

    /// Begins a transaction and calls [SessionWithDb::set_tid()](apllodb-shared-components::SessionWithDb::set_tid()).
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has already begun in this session.
    fn begin(sel: Self::Slf, session: Self::Sess) -> ApllodbResult<()>;

    /// Commit a transaction and calls [SessionWithDb::unset_tid()](apllodb-shared-components::SessionWithDb::unset_tid()).
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    fn commit(&mut self, session: Self::Sess) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction and calls [SessionWithDb::unset_tid()](apllodb-shared-components::SessionWithDb::unset_tid())..
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    fn abort(&mut self, session: Self::Sess) -> ApllodbResult<()>;
}
