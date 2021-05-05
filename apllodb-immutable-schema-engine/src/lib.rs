#![deny(missing_docs, missing_debug_implementations)]

//! An apllodb's storage engine implementation.

pub use apllodb_immutable_schema_engine_infra::ApllodbImmutableSchemaEngine;

#[cfg(test)]
pub mod tests {
    use apllodb_immutable_schema_engine_infra::test_support::test_setup;
    use ctor::ctor;

    #[ctor]
    fn setup() {
        test_setup();
    }
}
