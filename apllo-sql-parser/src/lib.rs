#![deny(warnings, missing_docs)]

//! APLLO SQL's syntax.
//!
//! # Interface
//! TBD
//!
//! Will input APLLO SQL and emit AST.

mod apllo_sql_parser;
mod parser_impl;
mod parser_interface;

pub use crate::apllo_sql_parser::AplloAST;
pub use crate::apllo_sql_parser::AplloSqlParser;
