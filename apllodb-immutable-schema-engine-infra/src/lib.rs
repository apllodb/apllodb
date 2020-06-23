//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

mod external_interface;
mod sqlite;

pub use external_interface::ApllodbImmutableSchemaEngine;
