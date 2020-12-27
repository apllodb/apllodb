#![deny(
    // warnings, // TODO on
    missing_debug_implementations, missing_docs
)]

//! TBD

#[macro_use]
extern crate derive_new;

pub(crate) mod query_executor;
pub(crate) mod query_plan;

#[cfg(test)]
pub(crate) mod test_support;
