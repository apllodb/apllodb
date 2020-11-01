//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Interface Adapter layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

mod transaction;

pub use transaction::TransactionController;
