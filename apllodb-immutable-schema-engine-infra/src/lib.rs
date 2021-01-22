//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

mod engine;

mod access_methods;
mod error;
mod immutable_schema_row_iter;
mod sqlite;

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

pub use crate::engine::ApllodbImmutableSchemaEngine;
