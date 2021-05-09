use std::collections::HashMap;

use apllodb_shared_components::{ApllodbError, ApllodbResult, SessionId};
use generational_arena::{Arena, Index};

use crate::sqlite::database::SqliteDatabase;

#[derive(Debug, Default)]
pub(crate) struct SqliteDatabasePool {
    pub(crate) db_arena: Arena<SqliteDatabase>,
    pub(crate) sess_db: HashMap<SessionId, Index>,
}

impl SqliteDatabasePool {
    /// # Failures
    ///
    /// - [ConnectionExceptionDatabaseNotOpen](apllodb-shared-components::SqlState::ConnectionExceptionDatabaseNotOpen) when:
    ///   - this session seems not to open any database.
    pub(crate) fn get_db(&self, sid: &SessionId) -> ApllodbResult<&SqliteDatabase> {
        let err = || {
            ApllodbError::connection_exception_database_not_open(format!(
                "session `{:?}` does not opens any database",
                sid
            ))
        };

        let db_idx = *self.sess_db.get(sid).ok_or_else(err)?;
        let db = self.db_arena.get(db_idx).ok_or_else(err)?;

        Ok(db)
    }

    /// # Failures
    ///
    /// - [ConnectionExceptionDatabaseNotOpen](apllodb-shared-components::SqlState::ConnectionExceptionDatabaseNotOpen) when:
    ///   - this session seems not to open any database.
    #[allow(dead_code)]
    pub(crate) fn remove_db(&mut self, sid: &SessionId) -> ApllodbResult<SqliteDatabase> {
        let err = || {
            ApllodbError::connection_exception_database_not_open(format!(
                "session `{:?}` does not open any database",
                sid
            ))
        };

        let db_idx = self.sess_db.remove(sid).ok_or_else(err)?;
        let db = self.db_arena.remove(db_idx).ok_or_else(err)?;

        Ok(db)
    }

    /// # Failures
    ///
    /// - [ConnectionExceptionDatabaseAlreadyOpen](apllodb-shared-components::SqlState::ConnectionExceptionDatabaseAlreadyOpen) when:
    ///   - this session seems to open another database.
    pub(crate) fn insert_db(&mut self, sid: &SessionId, db: SqliteDatabase) -> ApllodbResult<()> {
        let db_idx = self.db_arena.insert(db);
        if self.sess_db.insert(*sid, db_idx).is_some() {
            Err(ApllodbError::connection_exception_database_already_open(
                format!("session `{:?}` already opens another database", sid),
            ))
        } else {
            Ok(())
        }
    }
}
