use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::db_pool::SqliteDatabasePool};
use apllodb_shared_components::{DatabaseName, SessionId};
use apllodb_storage_engine_interface::WithoutDbMethods;
use futures::FutureExt;

use super::BoxFutRes;

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
    fn create_database_core(self, _sid: SessionId, database: DatabaseName) -> BoxFutRes<()> {
        async move {
            SqliteDatabase::create_database(database).await?;
            Ok(())
        }
        .boxed_local()
    }

    fn use_database_core(self, sid: SessionId, database: DatabaseName) -> BoxFutRes<()> {
        async move {
            let db = SqliteDatabase::use_database(database.clone()).await?;
            self.db_pool.borrow_mut().insert_db(&sid, db)?;

            Ok(())
        }
        .boxed_local()
    }
}
