//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

pub mod external_interface;

mod sqlite;

#[cfg(test)]
mod test_support;
