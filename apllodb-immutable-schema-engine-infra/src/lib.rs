#![deny(warnings,
    // missing_docs,
    missing_debug_implementations
)]

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
    use ctor::ctor;

    use crate::test_support::test_setup;

    #[ctor]
    fn setup() {
        test_setup();
    }
}
