#![deny(warnings, missing_docs, missing_debug_implementations)]

//! APLLO's storage manager interface.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllo-storage-manager-interface = "0.1"
//! ```
//!
//! By default, all public structs and enums implement Serde's `Serialize` and `Deserialize`
//! for storage engines to easily materialize them.
//! If your storage engine does not need them, use `default-features = false`.
//!
//! ```toml
//! [dependencies]
//! apllo-storage-manager-interface = { version = "0.1", default-features = false }
//! ```
//!
//! # Boundary of Responsibility with Storage Engine
//!
//! A storage engine is an implementation of this interface crate.
//!
//! This crate provides:
//!
//! - Access Methods traits related to:
//!   - APLLO DDL
//!   - APLLO DML
//!   - Transaction
//!   - Getting catalog
//! - Data structures and operations for them commonly used by every storage engine.
//!   - Version set
//!   - Version
//! - Traits of records and record iterators.
//! - Catalog data structure with read-only APIs.
//!
//! And a storage engine MUST provide:
//!
//! - Access Methods implementation.
//! - Ways to materialize version sets and versions.
//! - Implementation of records and record iterators.

mod access_methods;
mod versions;

pub use crate::access_methods::AccessMethodsDdl;
pub use crate::versions::{marker, Version, VersionSet};
