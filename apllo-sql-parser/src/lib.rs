#![deny(warnings, missing_docs, missing_debug_implementations)]

//! APLLO SQL's parser that inputs APLLO SQL and emit APLLO AST.
//!
//! # Example
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

mod apllo_sql_parser;
mod parser_impl;
mod parser_interface;

pub use crate::apllo_sql_parser::apllo_ast;
pub use crate::apllo_sql_parser::AplloAst;
pub use crate::apllo_sql_parser::AplloSqlParser;
