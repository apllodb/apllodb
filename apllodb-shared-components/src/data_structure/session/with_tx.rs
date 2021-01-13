use crate::SessionId;
use serde::{Deserialize, Serialize};

/// Session with open transaction.
///
/// Most SQL commands are executed via this type of session.
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct SessionWithTx {
    id: SessionId,
}

impl SessionWithTx {
    /// Construct a session with open database.
    ///
    /// A storage engine's implementation must call this after opening a database.
    pub(super) fn new(sid: SessionId) -> Self {
        Self { id: sid }
    }

    /// Get session ID
    pub fn get_id(&self) -> &SessionId {
        &self.id
    }
}
