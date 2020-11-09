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

mod abstract_types;
mod row;
mod transaction;

pub use crate::abstract_types::AbstractTypes;
pub use crate::row::{PrimaryKey, Row};
pub use crate::transaction::{Transaction, TransactionId};

use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};

/// An storage engine implementation must implement this.
pub trait StorageEngine<'tx, 'db: 'tx> {
    /// Specify database to use and return database object.
    fn use_database<Types: AbstractTypes<'tx, 'db>>(
        database_name: &DatabaseName,
    ) -> ApllodbResult<Types::Db>;

    /// Starts transaction and get transaction object.
    fn begin_transaction<Types: AbstractTypes<'tx, 'db>>(
        db: &'db mut Types::Db,
    ) -> ApllodbResult<Types::Tx> {
        Types::Tx::begin(db)
    }
}
