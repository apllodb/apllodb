#![deny(warnings, missing_debug_implementations)]

//! Data structures shared with multiple crates in the apllodb workspace.

pub mod data_structure;
pub mod error;
pub mod helper;
pub mod traits;

#[cfg(test)]
pub mod test_support;
