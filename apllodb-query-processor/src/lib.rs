#![deny(
    // warnings, // TODO on
    missing_debug_implementations, missing_docs
)]

//! TBD

#[macro_use]
extern crate derive_new;

//pub(crate) mod modification;
pub(crate) mod query;

#[cfg(test)]
pub(crate) mod test_support;
