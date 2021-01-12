use std::collections::HashMap;

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, TransactionId};

use crate::sqlite::transaction::sqlite_tx::SqliteTx;

#[derive(Debug, Default)]
pub(crate) struct TxRepo<'sess> {
    tx_repo: HashMap<TransactionId, SqliteTx<'sess>>,
}

impl<'sess> TxRepo<'sess> {
    pub(crate) fn insert(&mut self, tid: TransactionId, sqlite_tx: SqliteTx<'sess>) {
        self.tx_repo.insert(tid, sqlite_tx);
    }

    pub(crate) fn remove(&mut self, tid: &TransactionId) -> ApllodbResult<SqliteTx<'_>> {
        self.tx_repo.remove(tid).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::ConnectionDoesNotExist,
                format!(
                    "transaction id `{:?}` does not exist in database repository",
                    tid
                ),
                None,
            )
        })
    }

    pub(crate) fn get(&self, tid: &TransactionId) -> ApllodbResult<&SqliteTx<'sess>> {
        self.tx_repo.get(tid).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::ConnectionDoesNotExist,
                format!(
                    "transaction id `{:?}` does not exist in database repository",
                    tid
                ),
                None,
            )
        })
    }

    pub(crate) fn get_mut(&mut self, tid: &TransactionId) -> ApllodbResult<&mut SqliteTx<'sess>> {
        self.tx_repo.get_mut(tid).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::ConnectionDoesNotExist,
                format!(
                    "transaction id `{:?}` does not exist in database repository",
                    tid
                ),
                None,
            )
        })
    }
}
