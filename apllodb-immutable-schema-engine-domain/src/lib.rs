// #![deny(warnings, missing_docs, missing_debug_implementations)]

//! Domain layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

pub mod row;
pub mod row_iter;
pub mod traits;
pub mod transaction;
pub mod version;
pub mod vtable;

mod entity;

pub mod test_support;
