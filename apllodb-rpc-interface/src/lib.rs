mod response;

pub use response::ApllodbSuccess;

use apllodb_shared_components::{ApllodbResult, DatabaseName, Session, SessionWithTx};

#[tarpc::service]
pub trait ApllodbRpc {
    /// Just a utility function. TODO remove after introducing standard command to open database.
    async fn begin_transaction(database: DatabaseName) -> ApllodbResult<SessionWithTx>;

    async fn command(session: Session, sql: String) -> ApllodbResult<ApllodbSuccess>;
}
