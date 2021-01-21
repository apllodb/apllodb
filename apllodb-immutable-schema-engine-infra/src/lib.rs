//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

pub mod engine;

mod error;
mod immutable_schema_row_iter;
mod sqlite;

#[cfg(test)]
mod test_support;
