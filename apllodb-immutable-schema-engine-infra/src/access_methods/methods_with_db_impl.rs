use apllodb_shared_components::{ApllodbResult, SessionWithDb, SessionWithTx};
use apllodb_storage_engine_interface::MethodsWithDb;

use crate::{
    db_repo::DbRepo,
    sqlite::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx},
    tx_repo::TxRepo,
};

#[derive(Debug)]
pub struct MethodsWithDbImpl<'sess> {
    session: &'sess SessionWithDb,
    db_repo: &'sess mut DbRepo,
    tx_repo: &'sess mut TxRepo<'sess>,
}

impl<'sess> MethodsWithDbImpl<'sess> {
    pub(crate) fn new(
        session: &'sess SessionWithDb,
        db_repo: &'sess mut DbRepo,
        tx_repo: &'sess mut TxRepo<'sess>,
    ) -> Self {
        Self {
            session,
            db_repo,
            tx_repo,
        }
    }

    fn remove_sqlite_tx(&mut self, session: &mut SessionWithDb) -> ApllodbResult<SqliteTx> {
        let sid = { session.get_id().clone() };

        let sqlite_tx = self.tx_repo.remove(&sid).expect(&format!(
            "no one should remove tid `{:?}` from tx_repo",
            sid
        ));
        Ok(sqlite_tx)
    }
}

impl<'sess> MethodsWithDb for MethodsWithDbImpl<'sess> {
    fn begin(self) -> ApllodbResult<SessionWithTx> {
        let session = self.session;
        let sid = { session.get_id().clone() };

        let sqlite_db: &'sess mut SqliteDatabase = self.db_repo.get_mut(&sid)?;
        let sqlite_tx = SqliteTx::begin(sqlite_db)?;

        self.tx_repo.insert(sid, sqlite_tx);

        Ok(session.upgrade())
    }
}
