#![deny(warnings, missing_docs, missing_debug_implementations)]

//! APLLO SQL's parser that inputs APLLO SQL and emit APLLO AST.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! apllo-sql-parser = "0.1"
//! ```
//!
//! If you want to serialize `AplloAst`, enable `"serde"` feature flag.
//!
//! ```toml
//! [dependencies]
//! apllo-sql-parser = { version = "0.1", features = ["serde"] }
//! ```
//!
//! # Example
//!
//! ```
//! use apllo_sql_parser::apllo_ast::{Command, DropTableCommand, Identifier, TableName};
//! use apllo_sql_parser::{AplloAst, AplloSqlParser};
//!
//! let parser = AplloSqlParser::new();
//! match parser.parse("DROP TABLE people") {
//!     Ok(AplloAst(Command::DropTableCommandVariant(DropTableCommand {
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
//! # APLLO SQL Syntax
//!
//! APLLO SQL Syntax is defined solely in `src/pest_grammar/apllo_sql.pest`.
//! The syntax is written in [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar).
//!
//! This file is internally parsed by [pest](https://github.com/pest-parser/pest)
//! but the main purpose is to describe the APLLO SQL syntax for humans.
//!
//! # APIs Overview
//!
//! `AplloSqlParser` provides `new()` and `parse()`.
//! `AplloSqlParser::parse()` returns `Result<AplloAst, AplloSqlParserError>`.
//!
//! `AplloAst` has a straight-forward relationship with syntax definition.
//!
//! ![AplloAst example](https://user-images.githubusercontent.com/498788/81028439-5f2cf880-8ebc-11ea-8554-deb884438779.png)
//!
//! | PEG syntax      | Rust element                          | Diagram                                    |
//! | --------------- | ------------------------------------- | ------------------------------------------ |
//! | `a = { b / c }` | `enum A { BVariant(B), CVariant(C) }` | `[A] -- ::BVariant --> [B]`                |
//! | `a = { b ~ c }` | `struct A { b: B, c: C }`             | `[A] -- .b --> [B]`<br>`[A] -- .c --> [C]` |
//! | `a = { b? }`    | `struct A { b: Option<B> }`           | -                                          |
//! | `a = { b* }`    | `struct A { b: Vec<B> }`              | `[A] -- .bs --> [B][...]`                  |
//! | `a = { b+ }`    | `struct A { b: NonEmptyVec<B> }`      | -                                          |
//!
//! `AplloAst` and its children nodes do not provide descriptive methods like `.is_select()` and `.has_where_clause()`.
//! These methods are about SEMANTICS. **apllo-sql-parser crate provides only SYNTAX**.
//!
//! For details, please check API reference.

mod apllo_sql_parser;
mod parser_impl;
mod parser_interface;

pub use crate::apllo_sql_parser::apllo_ast;
pub use crate::apllo_sql_parser::error;
pub use crate::apllo_sql_parser::AplloAst;
pub use crate::apllo_sql_parser::AplloSqlParser;
