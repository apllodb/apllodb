use crate::AplloAST;

pub(crate) trait ParserLike {
    fn parse<S: Into<String>, E>(&self, apllo_sql: S) -> Result<AplloAST, E>;
}
