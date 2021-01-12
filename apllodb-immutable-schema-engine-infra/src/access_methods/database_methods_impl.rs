pub(crate) mod db_repo;

use apllodb_shared_components::{ApllodbResult, SessionWithDb};
use apllodb_storage_engine_interface::DatabaseMethods;

use crate::sqlite::database::SqliteDatabase;

use self::db_repo::DbRepo;

#[derive(Debug)]
pub struct DatabaseMethodsImpl<'acc> {
    db_repo: &'acc mut DbRepo, // TODO Call DbRepo::remove
}

impl<'acc> DatabaseMethodsImpl<'acc> {
    pub(crate) fn new(db_repo: &'acc mut DbRepo) -> Self {
        Self { db_repo }
    }
}

impl DatabaseMethods for DatabaseMethodsImpl<'_> {
    fn use_database_core(&mut self, session: &SessionWithDb) -> ApllodbResult<()> {
        let db = SqliteDatabase::use_database(session.get_db().clone())?;
        self.db_repo.insert(session.get_id().clone(), db);
        Ok(())
    }
}
