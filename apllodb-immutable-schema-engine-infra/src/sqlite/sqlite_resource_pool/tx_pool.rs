use std::{
    collections::HashMap,
    {cell::RefCell, rc::Rc},
};

use apllodb_shared_components::{ApllodbError, ApllodbResult, SessionId};
use generational_arena::{Arena, Index};

use crate::sqlite::transaction::sqlite_tx::SqliteTx;

/// Transaction pool to access transactions in a session across requests to storage engine.
///
/// Each resource is accessible via [SessionId](apllodb-shared-components::SessionId).
#[derive(Debug, Default)]
pub(crate) struct SqliteTxPool {
    pub(crate) tx_arena: Arena<Rc<RefCell<SqliteTx>>>,
    pub(crate) sess_tx: HashMap<SessionId, Index>,
}

impl SqliteTxPool {
    /// # Failures
    ///
    /// - [InvalidTransactionState](apllodb-shared-components::SqlState::InvalidTransactionState) when:
    ///   - this session seems not to open any transaction.
    pub(crate) fn get_tx(&self, sid: &SessionId) -> ApllodbResult<Rc<RefCell<SqliteTx>>> {
        let err = || {
            ApllodbError::invalid_transaction_state(format!(
                "session `{:?}` does not open any transaction",
                sid
            ))
        };

        let tx_idx = *self.sess_tx.get(sid).ok_or_else(err)?;
        let tx = self.tx_arena.get(tx_idx).ok_or_else(err)?;

        Ok(tx.clone())
    }

    /// # Failures
    ///
    /// - [InvalidTransactionState](apllodb-shared-components::SqlState::InvalidTransactionState) when:
    ///   - this session seems not to open any transaction.
    pub(crate) fn remove_tx(&mut self, sid: &SessionId) -> ApllodbResult<Rc<RefCell<SqliteTx>>> {
        let err = || {
            ApllodbError::invalid_transaction_state(format!(
                "session `{:?}` does not open any transaction",
                sid
            ))
        };

        let tx_idx = self.sess_tx.remove(sid).ok_or_else(err)?;
        let tx = self.tx_arena.remove(tx_idx).ok_or_else(err)?;

        Ok(tx)
    }

    /// # Failures
    ///
    /// - [InvalidTransactionState](apllodb-shared-components::SqlState::InvalidTransactionState) when:
    ///   - this session seems to open another transaction.
    pub(crate) fn insert_tx(
        &mut self,
        sid: &SessionId,
        tx: Rc<RefCell<SqliteTx>>,
    ) -> ApllodbResult<()> {
        let tx_idx = self.tx_arena.insert(tx);
        if self.sess_tx.insert(*sid, tx_idx).is_some() {
            Err(ApllodbError::invalid_transaction_state(format!(
                "session `{:?}` already opens another transaction",
                sid
            )))
        } else {
            Ok(())
        }
    }
}
