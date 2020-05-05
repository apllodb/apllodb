pub mod apllo_ast;
pub mod error;

pub use apllo_ast::AplloAst;

use crate::{parser_impl::PestParserImpl, parser_interface::ParserLike};
use error::AplloSqlParserResult;

/// The parser from APLLO SQL into APLLO AST.
#[derive(Clone, Hash, Debug)]
pub struct AplloSqlParser(PestParserImpl);

impl AplloSqlParser {
    /// Constructor.
    pub fn new() -> Self {
        Self(PestParserImpl::new())
    }

    /// Parses APLLO SQL into APLLO AST.
    ///
    /// # Examples
    ///
    /// ```
    /// use apllo_sql_parser::AplloSqlParser;
    ///
    /// let parser = AplloSqlParser::new();
    /// match parser.parse("DROP TABLE people") {
    ///     Ok(ast) => println!("Parsed AST: {:?}", ast),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloSqlParserResult<AplloAst> {
        Ok(self.0.parse(apllo_sql)?)
    }
}

impl Default for AplloSqlParser {
    fn default() -> Self {
        Self::new()
    }
}
