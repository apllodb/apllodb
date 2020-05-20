pub mod apllodb_ast;
pub mod error;

pub use apllodb_ast::ApllodbAst;

use crate::{parser_impl::PestParserImpl, parser_interface::ParserLike};
use error::ApllodbSqlParserResult;

/// The parser from apllodb-SQL into APLLO AST.
#[derive(Clone, Hash, Debug)]
pub struct ApllodbSqlParser(PestParserImpl);

impl ApllodbSqlParser {
    /// Constructor.
    pub fn new() -> Self {
        Self(PestParserImpl::new())
    }

    /// Parses apllodb-SQL into APLLO AST.
    ///
    /// # Panics
    ///
    /// Only when internal implementation encounters a bug.
    /// Please report to the authors.
    ///
    /// # Failures
    ///
    /// When failed to parse input str as apllodb-SQL.
    /// The str must include some syntax errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use apllodb_sql_parser::ApllodbSqlParser;
    ///
    /// let parser = ApllodbSqlParser::new();
    /// match parser.parse("DROP TABLE people") {
    ///     Ok(ast) => println!("Parsed AST: {:?}", ast),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn parse<S: Into<String>>(&self, apllodb_sql: S) -> ApllodbSqlParserResult<ApllodbAst> {
        Ok(self.0.parse(apllodb_sql)?)
    }
}

impl Default for ApllodbSqlParser {
    fn default() -> Self {
        Self::new()
    }
}
