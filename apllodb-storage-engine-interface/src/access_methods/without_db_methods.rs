use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, DatabaseName, Session, SessionId,
    SessionWithDb, SessionWithoutDb,
};
use futures::FutureExt;

use super::BoxFut;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithoutDbMethods: Sized + 'static {
    fn create_database(
        self,
        session: Session,
        database: DatabaseName,
    ) -> BoxFut<ApllodbSessionResult<Session>> {
        let sid = *session.get_id();
        async move {
            match self.create_database_core(sid, database).await {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, session)),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn create_database_core(
        self,
        sid: SessionId,
        database: DatabaseName,
    ) -> BoxFut<ApllodbResult<()>>;

    fn use_database(
        self,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> BoxFut<ApllodbSessionResult<SessionWithDb>> {
        let sid = *session.get_id();
        async move {
            match self.use_database_core(sid, database.clone()).await {
                Ok(_) => Ok(session.upgrade(database)),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn use_database_core(self, sid: SessionId, database: DatabaseName)
        -> BoxFut<ApllodbResult<()>>;
}
