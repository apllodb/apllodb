// #![deny(warnings, missing_docs, missing_debug_implementations)]

//! Domain layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

pub mod abstract_types;
pub mod entity;
pub mod query;
pub mod row;
pub mod row_iter;
pub mod version;
pub mod version_revision_resolver;
pub mod vtable;
