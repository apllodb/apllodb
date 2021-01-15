use apllodb_shared_components::{ApllodbResult, DatabaseName, RecordIterator};
use serde::{Deserialize, Serialize};

/// Successful response from apllodb-server
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ApllodbRpcSuccess {
    QueryResponse { records: RecordIterator },
    ModificationResponse,
    DDLResponse,
}

#[tarpc::service]
pub trait ApllodbRpc {
    /// Returns a greeting for name.
    async fn command(db: DatabaseName, sql: String) -> ApllodbResult<ApllodbRpcSuccess>;
}
