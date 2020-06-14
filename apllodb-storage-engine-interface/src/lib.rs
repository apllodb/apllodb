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

mod row;
mod transaction;

pub use crate::row::{Row, RowBuilder};
pub use crate::transaction::Transaction;

use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};

/// An storage engine implementation must implement this.
pub trait StorageEngine<'tx> {
    /// Transaction implementation.
    type Tx: Transaction<'tx>;

    /// Specify database to use and return database object.
    fn use_database(
        database_name: &DatabaseName,
    ) -> ApllodbResult<<Self::Tx as Transaction<'tx>>::Db>; // Want to mark result type as `Self::Tx::Db` but not possible for now: https://github.com/rust-lang/rust/issues/38078

    /// Starts transaction and get transaction object.
    fn begin_transaction(
        db: &'tx mut <Self::Tx as Transaction<'tx>>::Db,
    ) -> ApllodbResult<Self::Tx>;
}
