#![deny(
    warnings,
    // missing_docs,
    missing_debug_implementations
)]
// FIXME <https://github.com/darwin-education/apllodb/issues/90>
#![allow(clippy::await_holding_refcell_ref)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

mod access_methods;
mod engine;
mod error;
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
