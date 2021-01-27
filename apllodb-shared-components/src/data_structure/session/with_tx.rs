use crate::{DatabaseName, SessionId, SessionWithDb};
use serde::{Deserialize, Serialize};

/// Session with open transaction.
///
/// Most SQL commands are executed via this type of session.
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct SessionWithTx {
    id: SessionId,
    db: DatabaseName,
}

impl SessionWithTx {
    /// Construct a session with open database.
    ///
    /// A storage engine's implementation must call this after opening a database.
    pub(super) fn new(sid: SessionId, db: DatabaseName) -> Self {
        Self { id: sid, db }
    }

    /// Downgrade to `SessionWithDb`.
    pub fn downgrade(self) -> SessionWithDb {
        SessionWithDb::new(self.id, self.db)
    }

    /// Get session ID
    pub fn get_id(&self) -> &SessionId {
        &self.id
    }

    /// Get database name
    pub fn get_db_name(&self) -> &DatabaseName {
        &self.db
    }
}
