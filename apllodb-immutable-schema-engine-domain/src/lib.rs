// #![deny(warnings, missing_docs, missing_debug_implementations)]

//! Domain layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

pub mod abstract_types;
pub mod entity;
pub mod row;
pub mod row_iter;
pub mod version;
pub mod vtable;

#[cfg(test)]
pub mod test_support;
