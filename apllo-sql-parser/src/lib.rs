#![deny(warnings, missing_docs)]

//! APLLO SQL's parser that inputs APLLO SQL and emit APLLO AST.
//!
//! # Example
//! ```
//! use apllo_sql_parser::{AplloSqlParser, AplloAST};
//!
//! let parser = AplloSqlParser::new();
//! match parser.parse("DROP TABLE people") {
//!     Ok(AplloAST::DropTable { table_name }) => assert_eq!(table_name, "people"),
//!     Ok(ast) => panic!("Expectedly parsed as DropTable but parsed as: {:?}", ast),
//!     Err(e) => panic!("{}", e),
//! }
//! ```

mod apllo_sql_parser;
mod parser_impl;
mod parser_interface;

pub use crate::apllo_sql_parser::AplloAST;
pub use crate::apllo_sql_parser::AplloSqlParser;
