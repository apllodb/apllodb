use std::{
    collections::HashMap,
    {cell::RefCell, rc::Rc},
};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, SessionId};
use generational_arena::{Arena, Index};

use crate::sqlite::transaction::sqlite_tx::SqliteTx;

/// rusqlite's Connection and Transaction pool.
///
/// Each resource is accessible via [SessionId](apllodb-shared-components::SessionId).
#[derive(Debug, Default)]
pub struct SqliteTxPool {
    pub(crate) tx_arena: Arena<Rc<RefCell<SqliteTx>>>,
    pub(crate) sess_tx: HashMap<SessionId, Index>,
}

impl SqliteTxPool {
    /// # Failures
    ///
    /// - [InvalidTransactionState](apllodb-shared-components::ApllodbErrorKind::InvalidTransactionState) when:
    ///   - this session seems to open another transaction.
    pub(crate) fn insert_tx(
        &mut self,
        sid: &SessionId,
        tx: Rc<RefCell<SqliteTx>>,
    ) -> ApllodbResult<()> {
        let tx_idx = self.tx_arena.insert(tx);
        if self.sess_tx.insert(sid.clone(), tx_idx).is_some() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTransactionState,
                format!("session `{:?}` already opens another transaction", sid),
                None,
            ))
        } else {
            Ok(())
        }
    }
}
