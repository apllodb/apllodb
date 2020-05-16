//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Data structures shared with multiple crates in the apllo workspace.

pub mod data_structure;
pub mod error;

#[cfg(test)]
pub(crate) mod test_support;
