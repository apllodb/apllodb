//! Error types.

use std::error::Error;

pub(crate) type AplloSqlParserResult<T> = std::result::Result<T, AplloSqlParserError>;

/// Error during parsing APLLO SQL. So called syntax error.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AplloSqlParserError {
    apllo_sql: String,
    reason: String,
}

impl AplloSqlParserError {
    pub(crate) fn new<S1: Into<String>, S2: Into<String>>(apllo_sql: S1, reason: S2) -> Self {
        Self {
            apllo_sql: apllo_sql.into(),
            reason: reason.into(),
        }
    }
}

impl Error for AplloSqlParserError {}

impl std::fmt::Display for AplloSqlParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "failed to parse the following APLLO SQL '{}' ; reason: {}",
            self.apllo_sql, self.reason
        )
    }
}
