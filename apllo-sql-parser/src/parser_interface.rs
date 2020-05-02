use crate::AplloAST;

pub(crate) trait ParserLike {
    fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloAST;
}
