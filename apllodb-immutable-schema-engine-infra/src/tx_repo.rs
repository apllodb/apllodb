use std::collections::HashMap;

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, SessionId};

use crate::sqlite::transaction::sqlite_tx::SqliteTx;

#[derive(Debug, Default)]
pub(crate) struct TxRepo<'sess> {
    tx_repo: HashMap<SessionId, SqliteTx<'sess>>,
}

impl<'sess> TxRepo<'sess> {
    pub(crate) fn insert(&mut self, sid: SessionId, sqlite_tx: SqliteTx<'sess>) {
        self.tx_repo.insert(sid, sqlite_tx);
    }

    pub(crate) fn remove(&mut self, sid: &SessionId) -> ApllodbResult<SqliteTx<'_>> {
        self.tx_repo.remove(sid).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::ConnectionDoesNotExist,
                format!(
                    "session id `{:?}` does not exist in database repository",
                    sid
                ),
                None,
            )
        })
    }

    pub(crate) fn get(&self, sid: &SessionId) -> ApllodbResult<&SqliteTx<'sess>> {
        self.tx_repo.get(sid).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::ConnectionDoesNotExist,
                format!(
                    "session id `{:?}` does not exist in database repository",
                    sid
                ),
                None,
            )
        })
    }

    pub(crate) fn get_mut(&mut self, sid: &SessionId) -> ApllodbResult<&mut SqliteTx<'sess>> {
        self.tx_repo.get_mut(sid).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::ConnectionDoesNotExist,
                format!(
                    "session id `{:?}` does not exist in database repository",
                    sid
                ),
                None,
            )
        })
    }
}
