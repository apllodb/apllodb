#![deny(warnings, missing_docs, missing_debug_implementations)]

//! apllodb's original storage engine implementation.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllodb-immutable-schema-engine = "0.1"
//! ```
//!
//! This crate provides:
//!
//! - Immutable Schema.
//! - ACID transaction.
//!
//! # Architecture
//!
//! apllodb-immutable-schema applies [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
//! in order to safely replace transactions and buffer managers implementation (in Frameworks & Derivers layer)
//! without changing Immutable Schema feature (in Enterprise/Application Business Rules layers).
//!
//! Here is the diagram describing Clean Architecture used in `apllodb-immutable-schema-*` repositories.
//!
//! ![apllodb-immutable-schema-engine Clean Architecture](https://user-images.githubusercontent.com/498788/85363246-5b802e80-b55b-11ea-98ca-a3d97f68a53a.png)
//!
//! # Limitations
//!
//! `async-std` is the only tested async runtime for this storage engine.
//!
//! This engine internally uses `sqlx::Pool`, which seems not to work with tokio.

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
