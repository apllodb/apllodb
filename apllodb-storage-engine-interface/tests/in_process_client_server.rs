use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{StorageEngine, StorageEngineClient};
use futures::{future, prelude::*};
use tarpc::{
    client, context,
    server::{self, Handler},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct TestStorageEngine;

#[tarpc::server]
impl StorageEngine for TestStorageEngine {
    async fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        Ok(session.upgrade(database))
    }

    async fn begin_transaction(
        self,
        _: context::Context,
        session: SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        Ok(session.upgrade())
    }

    async fn commit_transaction(
        self,
        _: context::Context,
        _session: SessionWithTx,
    ) -> ApllodbResult<()> {
        Ok(())
    }

    async fn abort_transaction(
        self,
        _: context::Context,
        _session: SessionWithTx,
    ) -> ApllodbResult<()> {
        Ok(())
    }

    async fn create_table(
        self,
        _: context::Context,
        session: SessionWithTx,
        _table_name: TableName,
        _table_constraints: TableConstraints,
        _column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<SessionWithTx> {
        Ok(session)
    }
}

#[tokio::test]
async fn test_in_process_client() -> ApllodbResult<()> {
    let (client_transport, server_transport) = tarpc::transport::channel::unbounded();

    let server = server::new(server::Config::default())
        .incoming(stream::once(future::ready(server_transport)))
        .respond_with(TestStorageEngine.serve());

    tokio::spawn(server);

    let mut client =
        StorageEngineClient::new(client::Config::default(), client_transport).spawn()?;

    let session = client
        .use_database(
            context::current(),
            SessionWithoutDb::default(),
            DatabaseName::new("x")?,
        )
        .await??;

    let session = client
        .begin_transaction(context::current(), session)
        .await??;

    let session = client
        .create_table(
            context::current(),
            session,
            TableName::new("t")?,
            TableConstraints::default(),
            vec![],
        )
        .await??;

    client
        .commit_transaction(context::current(), session)
        .await??;

    Ok(())
}
