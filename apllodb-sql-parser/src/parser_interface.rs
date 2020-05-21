use crate::{apllodb_sql_parser::error::ApllodbSqlParserResult, ApllodbAst};

pub(crate) trait ParserLike {
    fn parse<S: Into<String>>(&self, apllodb_sql: S) -> ApllodbSqlParserResult<ApllodbAst>;
}
