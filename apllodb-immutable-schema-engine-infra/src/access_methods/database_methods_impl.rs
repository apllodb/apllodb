mod db_repo;

use apllodb_shared_components::{ApllodbResult, SessionWithDb};
use apllodb_storage_engine_interface::DatabaseMethods;

use crate::sqlite::database::SqliteDatabase;

use self::db_repo::DbRepo;

#[derive(Debug, Default)]
pub struct DatabaseMethodsImpl {
    db_repo: DbRepo, // TODO Call DbRepo::remove
}

impl DatabaseMethods for DatabaseMethodsImpl {
    fn use_database_core(&mut self, session: &SessionWithDb) -> ApllodbResult<()> {
        let db = SqliteDatabase::use_database(session.get_db().clone())?;
        self.db_repo.insert(session.get_id().clone(), db);
        Ok(())
    }
}
