#![deny(warnings, missing_docs, missing_debug_implementations)]

//! apllodb's storage manager interface.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllodb-storage-manager-interface = "0.1"
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

mod access_methods;
mod transaction;
mod database;

pub use crate::access_methods::AccessMethodsDdl;
pub use crate::transaction::TxCtxLike;
pub use crate::database::DbCtxLike;
