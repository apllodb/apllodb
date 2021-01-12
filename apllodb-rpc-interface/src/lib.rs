mod error;

pub use error::{ApllodbRpcError, ApllodbRpcResult};

use apllodb_shared_components::{DatabaseName, RecordIterator, Session};
use serde::{Deserialize, Serialize};

/// Successful response from apllodb-server
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ApllodbRpcSuccess {
    QueryResponse { records: RecordIterator },
    ModificationResponse,
    DDLResponse,
}

/// apllodb-server's RPC interface.
#[tarpc::service]
pub trait ApllodbRpc {
    /// Establish a session.
    ///
    /// Session takes two forms: [Session::WithDb](apllodb-shared-components::Session::WithDb) and [Session::WithoutDb](apllodb-shared-components::Session::WithoutDb).
    /// If you path a valid [DatabaseName](apllodb-shared-components::DatabaseName), `Session::WithDb` is established.
    /// and otherwise [Session::WithoutDb].
    async fn establish_session(database_name: Option<DatabaseName>) -> ApllodbRpcResult<Session>;

    /// Runs an SQL (general meaning; including `CREATE DATABASE` and so on).
    async fn command(session: Session, sql: String) -> ApllodbRpcResult<ApllodbRpcSuccess>;
}
