mod use_case;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_rpc_interface::{ApllodbRpc, ApllodbRpcSuccess};
use apllodb_shared_components::{ApllodbResult, DatabaseName, Session, SessionWithTx};
use futures::{Future, FutureExt};

use std::{net::SocketAddr, pin::Pin, rc::Rc};
use tarpc::context;
use use_case::UseCase;

#[derive(Clone, Debug)]
pub(crate) struct ApllodbServer {
    addr: SocketAddr,
    use_case: Rc<UseCase<ApllodbImmutableSchemaEngine>>,
}

impl ApllodbServer {
    pub(crate) fn new(addr: SocketAddr, engine: Rc<ApllodbImmutableSchemaEngine>) -> Self {
        let use_case = Rc::new(UseCase::new(engine));
        Self { addr, use_case }
    }
}

type FutRes<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>>>>;

impl ApllodbRpc for ApllodbServer {
    type BeginTransactionFut = FutRes<SessionWithTx>;
    type CommandFut = FutRes<ApllodbRpcSuccess>;

    fn begin_transaction(
        self,
        _: context::Context,
        database: DatabaseName,
    ) -> Self::BeginTransactionFut {
        async move { self.use_case.begin_transaction(database).await }.boxed_local()
    }

    fn command(self, _: context::Context, session: Session, sql: String) -> Self::CommandFut {
        async move { self.use_case.command(session, &sql).await }.boxed_local()
    }
}
