mod use_case;

use apllodb_rpc_interface::{ApllodbRpc, ApllodbRpcSuccess};
use apllodb_shared_components::{ApllodbResult, DatabaseName};

use std::net::SocketAddr;
use tarpc::context;
use use_case::UseCase;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(crate) struct ApllodbServer(SocketAddr);

#[tarpc::server]
impl ApllodbRpc for ApllodbServer {
    async fn command(
        self,
        _: context::Context,
        db: DatabaseName,
        sql: String,
    ) -> ApllodbResult<ApllodbRpcSuccess> {
        UseCase::command(db, &sql)
    }
}
