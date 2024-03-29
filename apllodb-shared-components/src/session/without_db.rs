use serde::{Deserialize, Serialize};

use crate::{DatabaseName, SessionId, SessionWithDb};

/// Session without open database.
///
/// Only limited SQL commands (`CREATE DATABASE`, for example) are executed via this type of session.
#[derive(Hash, Debug, Default, Serialize, Deserialize)]
pub struct SessionWithoutDb {
    id: SessionId,
}

impl SessionWithoutDb {
    /// Get session ID
    pub fn get_id(&self) -> &SessionId {
        &self.id
    }

    /// Upgrade to `SessionWithDb`.
    pub fn upgrade(self, db: DatabaseName) -> SessionWithDb {
        SessionWithDb::new(self.id, db)
    }
}
