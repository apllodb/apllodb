use apllodb_shared_components::{RecordIterator, Session, SessionWithTx};
use serde::{Deserialize, Serialize};

/// Successful result from [SQLProcessor](crate::SQLProcessor).
#[derive(Debug, Serialize, Deserialize)]
pub enum SQLProcessorSuccess {
    /// Response from SELECT command.
    QueryRes {
        /// Same session with input session
        session: SessionWithTx,
        /// Result records
        records: RecordIterator,
    },

    /// Response from INSERT/UPDATE/DELETE command.
    ModificationRes {
        /// Same session with input session
        session: SessionWithTx,
    },

    /// Response from DDL command.
    DDLRes {
        /// Same session with input session
        session: SessionWithTx,
    },

    /// Response from database command.
    DatabaseRes {
        /// Same session with input session
        session: Session,
    },
}
