use std::{error::Error, fmt::Display, sync::PoisonError};

use apllodb_shared_components::ApllodbError;
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

impl<T: Sync + Send + 'static> From<PoisonError<T>> for InfraError {
    fn from(e: PoisonError<T>) -> Self {
        Self(ApllodbError::system_error(
            "a thread get poisoned",
            Box::new(e),
        ))
    }
}

impl From<sqlx::Error> for InfraError {
    fn from(e: sqlx::Error) -> Self {
        let default = |sqlx_err| {
            Self(ApllodbError::system_error(
                "backend sqlx raised an error",
                Box::new(sqlx_err),
            ))
        };

        match &e {
            sqlx::Error::Database(db_err) => {
                // SQLite's error codes: <https://www.sqlite.org/rescode.html#primary_result_code_list>
                match db_err.code().unwrap_or_default().to_string().as_str() {
                    // FIXME SQLITE_BUSY does not always indicate a deadlock.
                    // test_latter_tx_is_waited(), for example, should not end up in TransactionRollbackDeadlock error.
                    "5" => Self(ApllodbError::transaction_rollback_deadlock(
                        "deadlock detected",
                    )),
                    "14" => Self(ApllodbError::name_error_not_found(
                        "failed to open database file",
                    )),
                    "1555" => Self(ApllodbError::integrity_constraint_unique_violation(
                        "duplicate value on primary key",
                    )),
                    _ => default(e),
                }
            }
            _ => default(e),
        }
    }
}
