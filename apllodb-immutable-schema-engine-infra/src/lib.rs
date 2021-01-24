//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

mod engine;

mod access_methods;
mod error;
mod immutable_schema_row_iter;
mod sqlite;

pub use crate::engine::ApllodbImmutableSchemaEngine;

#[cfg(test)]
mod tests {
    use apllodb_test_support::setup::setup_test_logger;
    use ctor::ctor;

    #[cfg_attr(test, ctor)]
    fn test_setup() {
        setup_test_logger();
    }
}
