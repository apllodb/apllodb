use crate::{DatabaseName, SessionId};
use serde::{Deserialize, Serialize};

use super::with_tx::SessionWithTx;

/// Session with open database.
///
/// This session means there is no open transaction.
/// Only limited SQLs are valid for this type of session (auto-commit is work for query-processor. It must Create this kind of session to realize auto-commit).
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct SessionWithDb {
    id: SessionId,
    db: DatabaseName,
}

impl SessionWithDb {
    /// Construct a session with open database.
    ///
    /// A storage engine's implementation must call this after opening a database.
    pub(super) fn new(sid: SessionId, db: DatabaseName) -> Self {
        Self { id: sid, db }
    }

    /// Upgrade to `SessionWithTx`.
    pub fn upgrade(self) -> SessionWithTx {
        SessionWithTx::new(self.id, self.db)
    }

    /// Get session ID
    pub fn get_id(&self) -> &SessionId {
        &self.id
    }

    /// Get database name
    pub fn database_name(&self) -> &DatabaseName {
        &self.db
    }
}
