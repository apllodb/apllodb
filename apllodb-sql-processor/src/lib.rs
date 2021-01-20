#![deny(warnings, missing_debug_implementations, missing_docs)]

//! SQL processor.
//!
//! Takes subtree of [ApllodbAst](apllodb_sql_parser::ApllodbAst) and executes SQL.
//! If passed SQL requires access to tables, SQL processor calls storage engine APIs.
//!
//! # Examples
//!
//! TODO link to tests/*.rs

#[macro_use]
extern crate derive_new;

pub(crate) mod ast_translator;
pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;

pub use ddl::DDLProcessor;
pub use modification::ModificationProcessor;
pub use query::QueryProcessor;

#[cfg(test)]
pub(crate) mod test_support;
