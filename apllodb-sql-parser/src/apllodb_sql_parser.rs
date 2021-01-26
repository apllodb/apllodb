pub mod apllodb_ast;
pub mod error;

pub use apllodb_ast::ApllodbAst;

use crate::parser_impl::PestParserImpl;
use error::ApllodbSqlParserResult;

/// The parser from apllodb-SQL into apllodb-AST.
#[derive(Clone, Hash, Debug, Default)]
pub struct ApllodbSqlParser(PestParserImpl);

impl ApllodbSqlParser {
    /// Parses apllodb-SQL into apllodb-AST.
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
    /// let parser = ApllodbSqlParser::default();
    /// match parser.parse("DROP TABLE people") {
    ///     Ok(ast) => println!("Parsed AST: {:?}", ast),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn parse<S: Into<String>>(&self, apllodb_sql: S) -> ApllodbSqlParserResult<ApllodbAst> {
        Ok(self.0.parse(apllodb_sql)?)
    }
}
