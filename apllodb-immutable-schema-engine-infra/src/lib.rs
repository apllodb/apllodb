//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

mod engine;

mod access_methods;
mod error;
mod immutable_schema_row_iter;
mod sqlite;

pub use crate::engine::ApllodbImmutableSchemaEngine;

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

#[cfg(test)]
mod tests {
    use apllodb_test_support::setup::setup_test_logger;
    use ctor::ctor;

    use crate::test_support::clean_test_sqlite3;

    pub fn test_setup() {
        setup_test_logger();
        clean_test_sqlite3();
    }

    #[ctor]
    fn setup() {
        test_setup();
    }
}
