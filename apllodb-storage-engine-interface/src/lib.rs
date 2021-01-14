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
//!
//! # Examples
//!
//! TODO link to tests/*.rs

pub(crate) mod access_methods;

pub use crate::access_methods::{
    methods_with_db::MethodsWithDb,
    methods_with_tx::{projection::ProjectionQuery, MethodsWithTx},
    methods_without_db::MethodsWithoutDb,
};

use std::fmt::Debug;

/// An storage engine implementation must implement this trait and included associated-types.
///
/// # Lifetime parameters
///
/// - `'sess`: shorthand of `'session`. Each access methods returned from this trait's associated functions (like [db()](Self::db))
///   knows lifetime of a session from `&'sess self` and returns an instance that may die at `'sess`'s drop.
pub trait StorageEngine<'sess>: Default + Debug + Sized {
    /// Access methods without open database.
    type MethWithoutDb: MethodsWithoutDb;

    /// Access methods with open database (without transaction).
    type MethWithDb: MethodsWithDb;

    /// Access methods with open transaction.
    type MethWithTx: MethodsWithTx;

    /// MethodsWithoutDb implementation.
    fn without_db(&'sess mut self) -> Self::MethWithoutDb;

    /// MethodsWithDb implementation.
    fn with_db(&'sess mut self) -> Self::MethWithDb;

    /// MethodsWithTx implementation.
    fn with_tx(&'sess mut self) -> Self::MethWithTx;
}

#[cfg(feature = "test-support")]
pub mod test_support;
