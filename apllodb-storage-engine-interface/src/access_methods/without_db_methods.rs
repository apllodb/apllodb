use apllodb_shared_components::{DatabaseName, SessionWithDb, SessionWithoutDb};

use super::FutRes;

pub trait WithoutDbMethods {
    fn use_database(
        self,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> FutRes<SessionWithDb>;
}
