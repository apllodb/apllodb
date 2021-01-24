#![deny(warnings, missing_debug_implementations, missing_docs)]

//! SQL processor.
//!
//! Takes subtree of [ApllodbAst](apllodb_sql_parser::ApllodbAst) and executes SQL.
//! If passed SQL requires access to tables, SQL processor calls storage engine APIs.

#[macro_use]
extern crate derive_new;

pub(crate) mod ast_translator;
pub(crate) mod sql_processor;

pub use sql_processor::SQLProcessor;

#[cfg(test)]
pub(crate) mod test_support;
