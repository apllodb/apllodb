use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::db_pool::SqliteDatabasePool};
use apllodb_shared_components::{DatabaseName, SessionWithDb, SessionWithoutDb};
use futures::FutureExt;

use super::FutRes;

#[derive(Clone, Debug, Default, new)]
pub struct WithoutDbMethodsImpl {
    db_pool: Rc<RefCell<SqliteDatabasePool>>,
}

impl WithoutDbMethodsImpl {
    pub fn use_database(
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
