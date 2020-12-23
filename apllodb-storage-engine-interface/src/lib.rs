#![deny(warnings, missing_docs, missing_debug_implementations)]

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

mod query;
mod row;
mod transaction;

use std::fmt::Debug;

pub use crate::query::projection::ProjectionQuery;
pub use crate::row::{pk::PrimaryKey, Row};
pub use crate::transaction::{Transaction, transaction_id::TransactionId};

use apllodb_shared_components::{
    data_structure::DatabaseName, error::ApllodbResult, traits::Database,
};

/// An storage engine implementation must implement this trait and included associated-types.
///
/// Lifetimes:
/// - `'tx`: Lifetime of a transaction's start (BEGIN) to end (COMMIT/ABORT).
/// - `'db`: Lifetime database's open to close.
pub trait StorageEngine<'tx, 'db: 'tx>: Sized + Debug {
    /// Transaction.
    type Tx: Transaction<'tx, 'db, Self> + 'tx;

    /// Transaction ID.
    type TID: TransactionId;

    /// Database.
    type Db: Database + 'db;

    /// Row.
    type R: Row;

    /// Iterator of `Self::R`s returned from [select()](crate::Transaction::select) method.
    type RowIter: Iterator<Item = Self::R> + Debug;

    /// Specify database to use and return database object.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<Self::Db>;

    /// Starts transaction and get transaction object.
    fn begin_transaction(db: &'db mut Self::Db) -> ApllodbResult<Self::Tx> {
        Self::Tx::begin(db)
    }
}
