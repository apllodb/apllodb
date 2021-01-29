use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{ApllodbError, Session};

/// Result type commonly used in apllodb workspace.
pub type ApllodbSessionResult<T> = Result<T, ApllodbSessionError>;

/// Contains [ApllodbError](crate::ApllodbError) and [Session](crate::Session) who causes the error.
///
/// Used for server implementation to keep a session after getting error.
#[derive(Debug, Serialize, Deserialize, new)]
pub struct ApllodbSessionError {
    /// Error
    pub err: ApllodbError,

    /// Session who causes this error
    pub session: Session,
}

impl Error for ApllodbSessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.err.source()
    }
}

impl Display for ApllodbSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl From<ApllodbSessionError> for ApllodbError {
    fn from(cmd_err: ApllodbSessionError) -> Self {
        cmd_err.err
    }
}
