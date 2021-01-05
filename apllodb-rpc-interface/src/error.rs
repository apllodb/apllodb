use std::{error::Error, fmt::Display};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind};
use serde::{Deserialize, Serialize};

/// Result type commonly used in apllodb-server and apllodb-client.
pub type ApllodbRpcResult<T> = Result<T, ApllodbRpcError>;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ApllodbRpcError {
    /// Machine-readable error type.
    pub kind: ApllodbErrorKind,

    /// Human-readable description of each error instance.
    pub desc: String,
}

impl Error for ApllodbRpcError {}

impl Display for ApllodbRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.desc)
    }
}

impl From<ApllodbError> for ApllodbRpcError {
    fn from(e: ApllodbError) -> Self {
        Self {
            kind: e.kind().clone(),
            desc: e.desc().to_string(),
        }
    }
}
