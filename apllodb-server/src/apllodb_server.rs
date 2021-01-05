mod use_case;

use apllodb_rpc_interface::{ApllodbRpc, ApllodbRpcError, ApllodbRpcResult, ApllodbRpcSuccess};
use apllodb_shared_components::DatabaseName;

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
    ) -> ApllodbRpcResult<ApllodbRpcSuccess> {
        UseCase::command(db, &sql).map_err(ApllodbRpcError::from)
    }
}
