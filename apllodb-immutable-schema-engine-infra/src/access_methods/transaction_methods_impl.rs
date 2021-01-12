pub(crate) mod tx_repo;

use apllodb_shared_components::{ApllodbResult, SessionWithDb, TransactionId};
use apllodb_storage_engine_interface::TransactionMethods;

use crate::sqlite::transaction::sqlite_tx::SqliteTx;

use self::tx_repo::TxRepo;

use super::database_methods_impl::db_repo::DbRepo;

#[derive(Debug)]
pub struct TransactionMethodsImpl<'sess> {
    db_repo: &'sess mut DbRepo,
    tx_repo: TxRepo<'sess>,
}

impl<'sess> TransactionMethodsImpl<'sess> {
    pub(crate) fn new(db_repo: &'sess mut DbRepo) -> Self {
        Self {
            db_repo,
            tx_repo: TxRepo::default(),
        }
    }

    fn remove_sqlite_tx(&mut self, session: &mut SessionWithDb) -> ApllodbResult<SqliteTx> {
        let tid = session.get_tid()?;
        let sqlite_tx = self.tx_repo.remove(tid).expect(&format!(
            "no one should remove tid `{:?}` from tx_repo",
            tid
        ));
        Ok(sqlite_tx)
    }
}

impl TransactionMethods for TransactionMethodsImpl<'_> {
    fn begin_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<TransactionId> {
        let sqlite_db = self.db_repo.get_mut(session.get_id())?;
        let sqlite_tx = SqliteTx::begin(sqlite_db)?;
        let tid = { sqlite_tx.tid() };
        self.tx_repo.insert(tid.clone(), sqlite_tx);
        Ok(tid)
    }

    fn commit_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        self.remove_sqlite_tx(session)?.commit()
    }

    fn abort_core(&mut self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        self.remove_sqlite_tx(session)?.abort()
    }
}
