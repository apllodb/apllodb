#![deny(warnings, missing_debug_implementations)]

//! apllodb's storage engine interface.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllodb-storage-engine-interface = "0.1"
//! ```
//!
//! # Boundary of Responsibility with Storage Engine
//!
//! A storage engine is an implementation of this interface crate.
//!
//! This crate provides:
//!
//! - Access Methods traits related to:
//!   - apllodb-DDL
//!   - apllodb-DML
//!   - Transaction
//!   - Getting catalog
//! - Traits of records and record iterators.
//! - Catalog data structure with read-only APIs.
//!
//! And a storage engine MUST provide:
//!
//! - Access Methods implementation.
//! - Implementation of records and record iterators.
//! - Ways to materialize tables and records.
//!
//! # Examples
//!
//! TODO link to tests/

mod projection_query;

pub use projection_query::ProjectionQuery;

use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use std::fmt::Debug;

/// Storage engine interface.
#[tarpc::service]
pub trait StorageEngine {
    // ========================================================================
    // Database
    // ========================================================================

    /// Open a database.
    async fn use_database(
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb>;

    // ========================================================================
    // Transaction
    // ========================================================================

    /// Begin a transaction.
    async fn begin_transaction(session: SessionWithDb) -> ApllodbResult<SessionWithTx>;

    /// Commit an open transaction.
    async fn commit_transaction(session: SessionWithTx) -> ApllodbResult<()>;

    /// Abort an open transaction.
    async fn abort_transaction(session: SessionWithTx) -> ApllodbResult<()>;

    // ========================================================================
    // DDL
    // ========================================================================

    async fn create_table(
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<SessionWithTx>;
}
