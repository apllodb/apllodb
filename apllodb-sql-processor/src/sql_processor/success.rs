use apllodb_shared_components::{RecordIterator, Session, SessionWithDb, SessionWithTx};
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

    /// Response from CREATE DATABASE command.
    CreateDatabaseRes {
        /// Same session with input session
        session: Session,
    },

    /// Response from USE DATABASE command.
    UseDatabaseRes {
        /// session with open database
        session: SessionWithDb,
    },

    /// Response from BEGIN command.
    BeginTransactionRes {
        /// session with open transaction
        session: SessionWithTx,
    },

    /// Response from COMMIT/ABORT command
    TransactionEndRes {
        /// session with closed transaction
        session: SessionWithDb,
    },
}
