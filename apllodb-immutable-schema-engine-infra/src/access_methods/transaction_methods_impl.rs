pub(crate) mod tx_repo;

use apllodb_shared_components::{ApllodbResult, SessionWithDb};
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
        let tid = { session.get_tid()?.clone() };
        session.unset_tid()?;

        let sqlite_tx = self.tx_repo.remove(&tid).expect(&format!(
            "no one should remove tid `{:?}` from tx_repo",
            tid
        ));
        Ok(sqlite_tx)
    }
}

impl<'sess> TransactionMethods for TransactionMethodsImpl<'sess> {
    type Sess = &'sess mut SessionWithDb;
    type RefSelf = &'sess mut Self;

    fn begin(slf: &'sess mut Self, session: &'sess mut SessionWithDb) -> ApllodbResult<()> {
        let sid = { session.get_id().clone() };
        let sqlite_db = slf.db_repo.get_mut(&sid)?;
        let sqlite_tx = SqliteTx::begin(sqlite_db)?;
        let tid = { sqlite_tx.tid() };

        session.set_tid(tid.clone())?;
        slf.tx_repo.insert(tid.clone(), sqlite_tx);

        Ok(())
    }

    fn commit(&mut self, session: &'sess mut SessionWithDb) -> ApllodbResult<()> {
        self.remove_sqlite_tx(session)?.commit()
    }

    fn abort(&mut self, session: &'sess mut SessionWithDb) -> ApllodbResult<()> {
        self.remove_sqlite_tx(session)?.abort()
    }
}
