use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb, SessionWithoutDb};
use apllodb_storage_engine_interface::MethodsWithoutDb;

use crate::{db_repo::DbRepo, sqlite::database::SqliteDatabase};

#[derive(Debug)]
pub struct MethodsWithoutDbImpl<'sess> {
    session: &'sess SessionWithoutDb,
    db_repo: &'sess mut DbRepo, // TODO Call DbRepo::remove
}

impl<'sess> MethodsWithoutDbImpl<'sess> {
    pub(crate) fn new(session: &'sess SessionWithoutDb, db_repo: &'sess mut DbRepo) -> Self {
        Self { session, db_repo }
    }
}

impl MethodsWithoutDb for MethodsWithoutDbImpl<'_> {
    fn use_database(self, database_name: DatabaseName) -> ApllodbResult<SessionWithDb> {
        let session = self.session;
        let sid = { session.get_id().clone() };

        let db = SqliteDatabase::use_database(database_name.clone())?;
        self.db_repo.insert(sid, db);

        Ok(session.upgrade(database_name))
    }
}
