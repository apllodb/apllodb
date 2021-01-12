use std::collections::HashMap;

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, SessionId};

use crate::sqlite::database::SqliteDatabase;

#[derive(Debug, Default)]
pub(crate) struct DbRepo {
    db_repo: HashMap<SessionId, SqliteDatabase>,
}

impl DbRepo {
    pub(crate) fn insert(&mut self, sid: SessionId, db: SqliteDatabase) {
        self.db_repo.insert(sid, db);
    }

    pub(crate) fn remove(&mut self, sid: &SessionId) -> ApllodbResult<SqliteDatabase> {
        self.db_repo.remove(sid).ok_or_else(|| {
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

    pub(crate) fn get_mut(&mut self, sid: &SessionId) -> ApllodbResult<&mut SqliteDatabase> {
        self.db_repo.get_mut(sid).ok_or_else(|| {
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
