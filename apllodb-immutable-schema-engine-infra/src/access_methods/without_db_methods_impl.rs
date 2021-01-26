use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::db_pool::SqliteDatabasePool};
use apllodb_shared_components::{DatabaseName, Session, SessionWithDb, SessionWithoutDb};
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
            SqliteDatabase::create_database(database).await?;
            Ok(session)
        }
        .boxed_local()
    }

    fn use_database(
        self,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> FutRes<SessionWithDb> {
        async move {
            let db = SqliteDatabase::use_database(database.clone()).await?;
            self.db_pool.borrow_mut().insert_db(session.get_id(), db)?;

            Ok(session.upgrade(database))
        }
        .boxed_local()
    }
}
