//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

extern crate derive_new;

pub mod external_interface;

mod immutable_schema_row_iter;
mod sqlite;

#[cfg(test)]
mod test_support;
