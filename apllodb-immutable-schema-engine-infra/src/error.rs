use std::{error::Error, fmt::Display, sync::PoisonError};

use apllodb_shared_components::{ApllodbError, SqlState};
use serde::{Deserialize, Serialize};

/// Glue error from implementation details into [ApllodbError](apllodb-shared-components::ApllodbError).
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct InfraError(ApllodbError);

impl Error for InfraError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl Display for InfraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<InfraError> for ApllodbError {
    fn from(e: InfraError) -> Self {
        e.0
    }
}

impl<T> From<PoisonError<T>> for InfraError {
    fn from(e: PoisonError<T>) -> Self {
        Self(ApllodbError::new(
            SqlState::SystemError,
            format!("a thread get poisoned: {}", e),
            None,
        ))
    }
}

impl From<sqlx::Error> for InfraError {
    fn from(e: sqlx::Error) -> Self {
        let default = |sqlx_err| {
            Self(ApllodbError::new(
                SqlState::IoError,
                "backend sqlx raised an error",
                Some(Box::new(sqlx_err)),
            ))
        };

        match &e {
            sqlx::Error::Database(db_err) => {
                // SQLite's error codes: <https://www.sqlite.org/rescode.html#primary_result_code_list>
                match db_err.code().unwrap_or_default().to_string().as_str() {
                    // FIXME SQLITE_BUSY does not always indicate a deadlock.
                    // test_latter_tx_is_waited(), for example, should not end up in DeadlockDetected error.
                    "5" => Self(ApllodbError::new(
                        SqlState::DeadlockDetected,
                        "deadlock detected",
                        Some(Box::new(e)),
                    )),
                    "14" => Self(ApllodbError::new(
                        SqlState::UndefinedObject,
                        "failed to open database file",
                        Some(Box::new(e)),
                    )),
                    "1555" => Self(ApllodbError::new(
                        SqlState::UniqueViolation,
                        "duplicate value on primary key",
                        Some(Box::new(e)),
                    )),
                    _ => default(e),
                }
            }
            _ => default(e),
        }
    }
}
