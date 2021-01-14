use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb, SessionWithoutDb};

#[cfg(feature = "test-support")]
use mockall::automock;

/// Database access methods interface.
#[cfg_attr(feature = "test-support", automock)]
pub trait MethodsWithoutDb {
    /// Start a session with a database open.
    fn use_database(
        self,
        session: SessionWithoutDb,
        database_name: DatabaseName,
    ) -> ApllodbResult<SessionWithDb>;
}
