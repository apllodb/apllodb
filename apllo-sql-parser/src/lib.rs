//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! APLLO SQL's parser that inputs APLLO SQL and emit APLLO AST.
//!
//! # Example
//! ```
//! use apllo_sql_parser::apllo_ast::{
//!     DropTableStatement, EmbeddedSqlStatement, Identifier, SqlExecutableStatement,
//!     SqlSchemaManipulationStatement, SqlSchemaStatement, StatementOrDeclaration,
//! };
//! use apllo_sql_parser::{AplloAst, AplloSqlParser};
//!
//! let parser = AplloSqlParser::new();
//! match parser.parse("DROP TABLE people") {
//!     Ok(AplloAst(EmbeddedSqlStatement {
//!         statement_or_declaration:
//!             StatementOrDeclaration::SqlExecutableStatementVariant(
//!                 SqlExecutableStatement::SqlSchemaStatementVariant(
//!                     SqlSchemaStatement::SqlSchemaManipulationStatementVariant(
//!                         SqlSchemaManipulationStatement::DropTableStatementVariant(
//!                             DropTableStatement {
//!                                 table_name: Identifier(table_name),
//!                             },
//!                         ),
//!                     ),
//!                 ),
//!             ),
//!     })) => assert_eq!(table_name, "people"),
//!
//!     Err(e) => panic!("{}", e),
//! }
//! ```

mod apllo_sql_parser;
mod parser_impl;
mod parser_interface;

pub use crate::apllo_sql_parser::apllo_ast;
pub use crate::apllo_sql_parser::AplloAst;
pub use crate::apllo_sql_parser::AplloSqlParser;
