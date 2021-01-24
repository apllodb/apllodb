//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

mod engine;

mod access_methods;
mod error;
mod immutable_schema_row_iter;
mod sqlite;

pub use crate::engine::ApllodbImmutableSchemaEngine;
