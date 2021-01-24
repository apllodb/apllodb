use apllodb_shared_components::{DatabaseName, SessionWithDb, SessionWithoutDb};

use super::FutRes;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithoutDbMethods {
    fn use_database(
        self,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> FutRes<SessionWithDb>;
}
