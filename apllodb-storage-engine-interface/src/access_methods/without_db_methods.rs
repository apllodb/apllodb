use apllodb_shared_components::{DatabaseName, Session, SessionWithDb, SessionWithoutDb};

use super::FutRes;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithoutDbMethods {
    fn create_database(self, session: Session, database: DatabaseName) -> FutRes<Session>;

    fn use_database(
        self,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> FutRes<SessionWithDb>;
}
