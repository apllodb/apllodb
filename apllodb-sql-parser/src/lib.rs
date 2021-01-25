#![deny(warnings, missing_docs, missing_debug_implementations)]

//! apllodb's parser that inputs apllodb-SQL and emit apllodb-AST.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllodb-sql-parser = "0.1"
//! ```
//!
//! If you want to serialize `ApllodbAst`, enable `"serde"` feature flag.
//!
//! ```toml
//! [dependencies]
//! apllodb-sql-parser = { version = "0.1", features = ["serde"] }
//! ```
//!
//! # Example
//!
//! ## Pattern matching `ApllodbAst`
//!
//! ```
//! use apllodb_sql_parser::apllodb_ast::{Command, DropTableCommand, Identifier, TableName};
//! use apllodb_sql_parser::{ApllodbAst, ApllodbSqlParser};
//!
//! let parser = ApllodbSqlParser::new();
//! match parser.parse("DROP TABLE people") {
//!     Ok(ApllodbAst(Command::DropTableCommandVariant(DropTableCommand {
//!         table_name: TableName(Identifier(table_name)),
//!     }))) => {
//!         assert_eq!(table_name, "people");
//!     }
//!     Ok(ast) => panic!(
//!         "Should be parsed as DROP TABLE but is parsed like: {:?}",
//!         ast
//!     ),
//!     Err(e) => panic!("{}", e),
//! }
//! ```
//!
//! ## Error handling
//!
//! ```
//! use apllodb_sql_parser::ApllodbSqlParser;
//! use std::error::Error;
//!
//! let parser = ApllodbSqlParser::new();
//! match parser.parse("DROP TABLE FROM people") {
//!     Err(e) => {
//!         assert!(e.source().is_none(), "No root cause. Just a syntax error.");
//!         eprintln!("Error detail: {}", e);
//!     }
//!     Ok(ast) => panic!("Syntax error should be reported but parsed as: {:?}", ast),
//! }
//! ```
//!
//! # apllodb-SQL Syntax
//!
//! apllodb-SQL Syntax is defined solely in `src/pest_grammar/apllodb_sql.pest`.
//! The syntax is written in [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar).
//!
//! This file is internally parsed by [pest](https://github.com/pest-parser/pest)
//! but the main purpose is to describe the apllodb-SQL syntax for humans.
//!
//! # APIs Overview
//!
//! `ApllodbSqlParser` provides `new()` and `parse()`.
//! `ApllodbSqlParser::parse()` returns `Result<ApllodbAst, ApllodbSqlParserError>`.
//!
//! `ApllodbAst` has a straight-forward relationship with syntax definition.
//!
//! ![ApllodbAst example](https://user-images.githubusercontent.com/498788/81168674-63e3d080-8fd2-11ea-9fd2-151d42fd0ad0.png)
//!
//! | PEG syntax      | Rust element                          | Diagram                                    |
//! | --------------- | ------------------------------------- | ------------------------------------------ |
//! | `a = { b / c }` | `enum A { BVariant(B), CVariant(C) }` | `[A] -- ::BVariant --> [B]`                |
//! | `a = { b ~ c }` | `struct A { b: B, c: C }`             | `[A] -- .b --> [B]`<br>`[A] -- .c --> [C]` |
//! | `a = { b? }`    | `struct A { b: Option<B> }`           | -                                          |
//! | `a = { b* }`    | `struct A { b: Vec<B> }`              | `[A] -- .bs --> [B][...]`                  |
//! | `a = { b+ }`    | `struct A { b: NonEmptyVec<B> }`      | -                                          |
//!
//! `ApllodbAst` and its children nodes do not provide descriptive methods like `.is_select()` and `.has_where_clause()`.
//! These methods are about SEMANTICS. **apllodb-sql-parser crate provides only SYNTAX**.
//!
//! For details, please check API reference.

mod apllodb_sql_parser;
mod parser_impl;
mod parser_interface;

pub use crate::apllodb_sql_parser::apllodb_ast;
pub use crate::apllodb_sql_parser::error;
pub use crate::apllodb_sql_parser::ApllodbAst;
pub use crate::apllodb_sql_parser::ApllodbSqlParser;

#[cfg(feature = "test-support")]
#[allow(missing_docs)]
pub mod test_support;

#[cfg(test)]
mod tests {
    use apllodb_test_support::setup::setup_test_logger;
    use ctor::ctor;

    #[cfg_attr(test, ctor)]
    fn test_setup() {
        setup_test_logger();
    }
}
