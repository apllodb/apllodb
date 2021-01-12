use std::collections::HashMap;

use apllodb_shared_components::{ApllodbResult, SessionId, SessionWithDb, TransactionId};
use apllodb_storage_engine_interface::TransactionMethods;

use crate::sqlite::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx};

#[derive(Debug, Default)]
pub struct TransactionMethodsImpl<'sess> {
    db_repo: &'sess HashMap<SessionId, SqliteDatabase>, // TODO wrap with type
    tx_repo: HashMap<TransactionId, SqliteTx<'sess>>,
}

impl<'sess> TransactionMethods for TransactionMethodsImpl<'sess> {
    fn begin_core(&self, session: &'sess mut SessionWithDb) -> ApllodbResult<TransactionId> {
        todo!()
    }

    fn commit_core(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        todo!()
    }

    fn abort_core(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        todo!()
    }
}
