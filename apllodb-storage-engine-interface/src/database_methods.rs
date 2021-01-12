use std::fmt::Debug;

use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb};

/// Database access methods interface.
pub trait DatabaseMethods: Debug {
    /// Start a session with a database open.
    fn use_database(&mut self, database_name: DatabaseName) -> ApllodbResult<SessionWithDb> {
        let session = SessionWithDb::new(database_name.clone());
        self.use_database_core(&session)?;
        Ok(session)
    }

    #[doc(hidden)]
    fn use_database_core(&mut self, session: &SessionWithDb) -> ApllodbResult<()>;
}
