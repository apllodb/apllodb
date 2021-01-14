use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb};

#[cfg(any(test, feature = "test_support"))]
use mockall::automock;

/// Database access methods interface.
#[cfg_attr(any(test, feature = "test_support"), automock)]
pub trait MethodsWithoutDb {
    /// Start a session with a database open.
    fn use_database(self, database_name: DatabaseName) -> ApllodbResult<SessionWithDb>;
}
