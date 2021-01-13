use std::fmt::Debug;

use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb};

/// Database access methods interface.
pub trait DatabaseMethods: Debug + Sized {
    /// Start a session with a database open.
    fn use_database(self, database_name: DatabaseName) -> ApllodbResult<SessionWithDb>;
}
