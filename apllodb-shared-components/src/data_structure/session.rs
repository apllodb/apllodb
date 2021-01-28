//! Session information.
//!
//! Initially created from database client and used through server, sql-processor, and storage-engine.
//!
//! A session holds these information:
//!
//! - Open database (0/1)
//! - Beginning transaction (0/1 if a database is open; 0 if any database isn't open)
//!
//! Only storage-engine has direct access to database and transaction.
//! Other components create/modify/get database and transaction through access methods' call with session.
//!
//! Note that session is free from physical connection implementation.
//! Therefore, for example, client-server's transport is independent from Session and can be any of TCP, direct method call, and so on.

pub(crate) mod session_id;
pub(crate) mod with_db;
pub(crate) mod with_tx;
pub(crate) mod without_db;

use crate::{SessionId, SessionWithDb, SessionWithTx, SessionWithoutDb};

use serde::{Deserialize, Serialize};

/// Session types
#[derive(Hash, Debug, Serialize, Deserialize)]
pub enum Session {
    /// session without open database
    WithoutDb(SessionWithoutDb),

    /// session with open database
    WithDb(SessionWithDb),

    /// session with open transaction
    WithTx(SessionWithTx),
}

impl From<SessionWithoutDb> for Session {
    fn from(s: SessionWithoutDb) -> Self {
        Session::WithoutDb(s)
    }
}
impl From<SessionWithDb> for Session {
    fn from(s: SessionWithDb) -> Self {
        Session::WithDb(s)
    }
}
impl From<SessionWithTx> for Session {
    fn from(s: SessionWithTx) -> Self {
        Session::WithTx(s)
    }
}

impl Session {
    /// get session ID
    pub fn get_id(&self) -> &SessionId {
        match self {
            Session::WithoutDb(s) => s.get_id(),
            Session::WithDb(s) => s.get_id(),
            Session::WithTx(s) => s.get_id(),
        }
    }
}
