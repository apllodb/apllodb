pub type AplloSqlParserResult<T> = std::result::Result<T, AplloSqlParserError>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AplloSqlParserError {
    apllo_sql: String,
    reason: String,
}

impl std::fmt::Display for AplloSqlParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Failed to parser the following APLLO SQL: {}\
            Reason: {}\
            ",
            self.apllo_sql, self.reason
        )
    }
}
