use std::collections::HashMap;

use apllodb_shared_components::{ApllodbResult, SessionId, SessionWithDb, TransactionId};
use apllodb_storage_engine_interface::TransactionMethods;

use crate::sqlite::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx};

use super::database_methods_impl::db_repo::DbRepo;

#[derive(Debug)]
pub struct TransactionMethodsImpl<'sess> {
    db_repo: &'sess DbRepo,
    tx_repo: HashMap<TransactionId, SqliteTx<'sess>>,
}

impl<'sess> TransactionMethodsImpl<'sess> {
    pub(crate) fn new(db_repo: &'sess DbRepo) -> Self {
        Self {
            db_repo,
            tx_repo: HashMap::new(),
        }
    }
}

impl TransactionMethods for TransactionMethodsImpl<'_> {
    fn begin_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<TransactionId> {
        let sqlite_tx = SqliteTx::begin(self.db_repo.get_mut(session.get_id())?)?;
        let tid = { sqlite_tx.tid() };
        self.tx_repo.insert(tid.clone(), sqlite_tx);
        Ok(tid)
    }

    fn commit_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        todo!()
    }

    fn abort_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        todo!()
    }
}
