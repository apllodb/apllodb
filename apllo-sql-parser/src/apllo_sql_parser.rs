mod apllo_ast;

pub use apllo_ast::AplloAST;

use crate::{parser_impl::PestParserImpl, parser_interface::ParserLike};

/// The parser from APLLO SQL into APLLO AST.
pub struct AplloSqlParser(PestParserImpl);

impl AplloSqlParser {
    /// Constructor.
    pub fn new() -> Self {
        Self(PestParserImpl::new())
    }

    /// Parses APLLO SQL into APLLO AST.
    ///
    /// # Panics
    ///
    /// # Failures
    ///
    /// # Safety
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloAST {
        self.0.parse(apllo_sql)
    }
}
