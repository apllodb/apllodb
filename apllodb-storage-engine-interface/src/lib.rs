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

mod access_methods;
mod projection_query;

#[cfg(feature = "test-support")]
pub mod test_support;

pub use access_methods::{
    with_db_methods::WithDbMethods, with_tx_methods::WithTxMethods,
    without_db_methods::WithoutDbMethods,
};
pub use projection_query::ProjectionQuery;

/// Storage engine interface.
pub trait StorageEngine {
    /// Access methods that take [SessionWithoutDb](apllodb-shared-components::SessionWithoutDb).
    type WithoutDb: WithoutDbMethods;

    /// Access methods that take [SessionWithDb](apllodb-shared-components::SessionWithDb).
    type WithDb: WithDbMethods;

    /// Access methods that take [SessionWithTx](apllodb-shared-components::SessionWithTx).
    type WithTx: WithTxMethods;

    fn without_db(&self) -> Self::WithoutDb;

    fn with_db(&self) -> Self::WithDb;

    fn with_tx(&self) -> Self::WithTx;
}
