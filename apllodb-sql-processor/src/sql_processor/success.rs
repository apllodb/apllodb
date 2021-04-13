use apllodb_shared_components::{Session, SessionWithDb, SessionWithTx};

use crate::records::Records;

/// Successful result from [SqlProcessor](crate::SqlProcessor).
#[derive(Debug)]
pub enum SqlProcessorSuccess {
    /// Response from SELECT command.
    QueryRes {
        /// Same session with input session
        session: SessionWithTx,
        /// Result records
        records: Records,
    },

    /// Response from INSERT/UPDATE/DELETE command.
    ModificationRes {
        /// Same session with input session
        session: SessionWithTx,
    },

    /// Response from DDL command.
    DdlRes {
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
