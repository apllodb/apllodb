use crate::{
    apllo_ast::AplloAST, parser_impl::pest_parser_impl::PestParserImpl,
    parser_interface::ParserLike,
};

pub struct AplloSqlParser(PestParserImpl);

impl AplloSqlParser {
    pub fn new() -> Self {
        Self(PestParserImpl::new())
    }

    pub fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloAST {
        self.0.parse(apllo_sql)
    }
}
