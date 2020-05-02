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
    fn parse<S: Into<String>, E>(&self, _apllo_sql: S) -> Result<AplloAST, E> {
        // TODO: 型合わせではなくちゃんと実装する
        Ok(AplloAST::DropTable {
            table_name: "people".into(),
        })
    }
}
