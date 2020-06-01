#![deny(warnings, missing_docs, missing_debug_implementations)]

//! apllodb's original storage engine implementation.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllodb-immutable-schema-engine = "0.1"
//! ```
//!
//! This crate provides:
//!
//! - Immutable Schema.
//! - ACID transaction (with poor performance) working with SERIALIZABLE isolation level.

mod access_methods;
mod helper;
mod latch;
mod table;
mod transaction;
mod version;

pub use crate::access_methods::AccessMethods;
pub use crate::table::Table;
pub use crate::version::{ActiveVersion, InactiveVersion};

#[cfg(test)]
pub(crate) mod test_support;
