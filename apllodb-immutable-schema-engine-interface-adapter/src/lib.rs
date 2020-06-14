//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Interface Adapter layer of apllodb-immutable-schema-engine.

mod transaction;

pub use transaction::TransactionController;
