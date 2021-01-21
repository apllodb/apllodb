use std::{cell::RefCell, collections::HashMap, rc::Rc};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, SessionId};
use generational_arena::{Arena, Index};

use super::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx};

/// rusqlite's Connection and Transaction pool.
///
/// Each resource is accessible via [SessionId](apllodb-shared-components::SessionId).
#[derive(Debug, Default)]
pub(crate) struct SqliteResourcePool {
    pub(crate) db_arena: Arena<SqliteDatabase>,
    pub(crate) tx_arena: Arena<Rc<RefCell<SqliteTx>>>,

    pub(crate) sess_db: HashMap<SessionId, Index>,
    pub(crate) sess_tx: HashMap<SessionId, Index>,
}

impl SqliteResourcePool {
    /// # Failures
    ///
    /// - [UndefinedObject](apllodb-shared-components::ApllodbErrorKind::UndefinedObject) when:
    ///   - this session seems not to open any database.
    pub(crate) fn get_db_mut(&mut self, sid: &SessionId) -> ApllodbResult<&mut SqliteDatabase> {
        let err = || {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedObject,
                format!("session `{:?}` does not opens any database", sid),
                None,
            )
        };

        let db_idx = self.sess_db.get(sid).ok_or_else(err)?.clone();
        let db = self.db_arena.get_mut(db_idx).ok_or_else(err)?;

        Ok(db)
    }

    /// # Failures
    ///
    /// - [DuplicateDatabase](apllodb-shared-components::ApllodbErrorKind::DuplicateDatabase) when:
    ///   - this session seems to open another database.
    pub(crate) fn insert_db(&mut self, sid: &SessionId, db: SqliteDatabase) -> ApllodbResult<()> {
        let db_idx = self.db_arena.insert(db);
        if self.sess_db.insert(sid.clone(), db_idx).is_some() {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateDatabase,
                format!("session `{:?}` already opens another database", sid),
                None,
            ))
        } else {
            Ok(())
        }
    }

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
