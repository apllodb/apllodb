use apllodb_shared_components::{ApllodbResult, SessionWithDb, SessionWithTx};
use apllodb_storage_engine_interface::MethodsWithDb;

use crate::{
    db_repo::DbRepo,
    sqlite::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx},
    tx_repo::TxRepo,
};

#[derive(Debug)]
pub struct MethodsWithDbImpl<'sess> {
    db_repo: &'sess mut DbRepo,
    tx_repo: &'sess mut TxRepo<'sess>,
}

impl<'sess> MethodsWithDbImpl<'sess> {
    pub(crate) fn new(db_repo: &'sess mut DbRepo, tx_repo: &'sess mut TxRepo<'sess>) -> Self {
        Self { db_repo, tx_repo }
    }
}

impl<'sess> MethodsWithDb for MethodsWithDbImpl<'sess> {
    fn begin(self, session: SessionWithDb) -> ApllodbResult<SessionWithTx> {
        let sid = { session.get_id().clone() };

        let sqlite_db: &'sess mut SqliteDatabase = self.db_repo.get_mut(&sid)?;
        let sqlite_tx = SqliteTx::begin(sqlite_db)?;

        self.tx_repo.insert(sid, sqlite_tx);

        Ok(session.upgrade())
    }
}
