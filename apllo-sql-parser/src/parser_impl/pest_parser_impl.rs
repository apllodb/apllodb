mod generated_parser;

use crate::{parser_interface::ParserLike, AplloAST};

#[derive(Clone, Hash, Debug)]
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
