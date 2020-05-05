use crate::{apllo_sql_parser::error::AplloSqlParserResult, AplloAst};

pub(crate) trait ParserLike {
    fn parse<S: Into<String>>(&self, apllo_sql: S) -> AplloSqlParserResult<AplloAst>;
}
