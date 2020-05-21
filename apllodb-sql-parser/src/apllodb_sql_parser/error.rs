//! Error types.

use std::error::Error;

pub(crate) type ApllodbSqlParserResult<T> = std::result::Result<T, ApllodbSqlParserError>;

/// Error during parsing apllodb-SQL. So called syntax error.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ApllodbSqlParserError {
    apllodb_sql: String,
    reason: String,
}

impl ApllodbSqlParserError {
    pub(crate) fn new<S1: Into<String>, S2: Into<String>>(apllodb_sql: S1, reason: S2) -> Self {
        Self {
            apllodb_sql: apllodb_sql.into(),
            reason: reason.into(),
        }
    }
}

impl Error for ApllodbSqlParserError {}

impl std::fmt::Display for ApllodbSqlParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "failed to parse the following apllodb-SQL '{}' ; reason: {}",
            self.apllodb_sql, self.reason
        )
    }
}
