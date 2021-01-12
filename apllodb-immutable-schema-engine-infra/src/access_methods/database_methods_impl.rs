use std::collections::HashMap;

use apllodb_shared_components::{ApllodbResult, SessionId, SessionWithDb};
use apllodb_storage_engine_interface::DatabaseMethods;

use crate::sqlite::database::SqliteDatabase;

#[derive(Debug, Default)]
pub struct DatabaseMethodsImpl {
    db_repo: HashMap<SessionId, SqliteDatabase>,
}

impl DatabaseMethods for DatabaseMethodsImpl {
    fn use_database_core(&mut self, session: &SessionWithDb) -> ApllodbResult<()> {
        let db = SqliteDatabase::use_database(session.get_db().clone())?;
        self.db_repo.insert(session.get_id().clone(), db);
        Ok(())
    }
}
