use std::{error::Error, fmt::Display, sync::PoisonError};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind};
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
            ApllodbErrorKind::SystemError,
            format!("a thread get poisoned: {}", e),
            None,
        ))
    }
}

impl From<sqlx::Error> for InfraError {
    fn from(e: sqlx::Error) -> Self {
        Self(ApllodbError::new(
            ApllodbErrorKind::IoError,
            "backend sqlx raised an error",
            Some(Box::new(e)),
        ))
    }
}
