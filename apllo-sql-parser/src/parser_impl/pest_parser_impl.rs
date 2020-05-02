mod generated_parser;

use crate::{apllo_ast::AplloAST, parser_interface::ParserLike};

pub(crate) struct PestParserImpl;

impl PestParserImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl ParserLike for PestParserImpl {
    fn parse<S: Into<String>>(&self, _apllo_sql: S) -> AplloAST {
        todo!()
    }
}
