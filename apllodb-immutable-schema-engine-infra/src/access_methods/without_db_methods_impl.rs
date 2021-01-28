use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::db_pool::SqliteDatabasePool};
use apllodb_shared_components::{
    ApllodbSessionError, DatabaseName, Session, SessionWithDb, SessionWithoutDb,
};
use apllodb_storage_engine_interface::WithoutDbMethods;
use futures::FutureExt;

use super::FutRes;

#[derive(Clone, Debug, Default)]
pub struct WithoutDbMethodsImpl {
    db_pool: Rc<RefCell<SqliteDatabasePool>>,
}

impl WithoutDbMethodsImpl {
    pub(crate) fn new(db_pool: Rc<RefCell<SqliteDatabasePool>>) -> Self {
        Self { db_pool }
    }
}

impl WithoutDbMethods for WithoutDbMethodsImpl {
    fn create_database(self, session: Session, database: DatabaseName) -> FutRes<Session> {
        async move {
            SqliteDatabase::create_database(database)
                .await
                .map_err(|e| ApllodbSessionError::new(e, Session::from(session)))?;
            Ok(session)
        }
        .boxed_local()
    }

    fn use_database(
        self,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> FutRes<SessionWithDb> {
        let sid = session.get_id().clone();
        async move {
            match SqliteDatabase::use_database(database.clone())
                .await
                .and_then(|db| self.db_pool.borrow_mut().insert_db(&sid, db))
            {
                Ok(_) => Ok(session.upgrade(database)),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }
}
