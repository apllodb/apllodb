use crate::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName, SessionId, TransactionId,
};
use serde::{Deserialize, Serialize};

/// Session with open database.
///
/// Most SQL commands are executed via this type of session.
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct SessionWithDb {
    id: SessionId,
    db: DatabaseName,
    tid: Option<TransactionId>,
}

impl SessionWithDb {
    /// Construct a session with open database.
    ///
    /// A storage engine's implementation must call this after opening a database.
    #[doc(hidden)]
    pub fn new(db: DatabaseName) -> Self {
        Self {
            id: Self::new_sid(),
            db,
            tid: None,
        }
    }

    // FIXME: Fast unique session ID generation is not a trivial task.
    // Don't do it in shared-components.
    fn new_sid() -> SessionId {
        let r = fastrand::u64(..);
        SessionId::new(r)
    }

    /// Set a TransactionId begun into this session.
    ///
    /// A storage engine's implementation must call this after beginning a transaction.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has already begun in this session.
    #[doc(hidden)]
    pub fn set_tid(&mut self, tid: TransactionId) -> ApllodbResult<()> {
        if self.tid.is_some() {
            return Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                format!("transaction has already begun: {:#?}", self.tid),
                None,
            ));
        }

        self.tid.replace(tid);

        Ok(())
    }

    /// Unset a TransactionId from this session.
    ///
    /// A storage engine's implementation must call this as soon as the associated transaction is committed/aborted.
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    #[doc(hidden)]
    pub fn unset_tx(&mut self) -> ApllodbResult<()> {
        self.tid.take().map(|_| ()).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                "transaction has not begun: {:#?}",
                None,
            )
        })
    }

    /// Get session ID
    pub fn get_id(&self) -> &SessionId {
        &self.id
    }

    /// Get ref to [DatabaseName](apllodb-shared-components::DatabaseName).
    pub fn get_db(&self) -> &DatabaseName {
        &self.db
    }

    /// Get ref to [TransactionId](apllodb-shared-components::TransactionId).
    ///
    /// # Failures
    ///
    /// - [InvalidTransactionState](crate::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - transaction has not begun in this session.
    pub fn get_tx(&self) -> ApllodbResult<&TransactionId> {
        self.tid.as_ref().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                "transaction has not begun: {:#?}",
                None,
            )
        })
    }
}
