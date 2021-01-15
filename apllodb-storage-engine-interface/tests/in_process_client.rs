use std::io;

use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb, SessionWithoutDb};
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
}

#[tokio::test]
async fn test_in_process_client() -> io::Result<()> {
    let (client_transport, server_transport) = tarpc::transport::channel::unbounded();

    let server = server::new(server::Config::default())
        .incoming(stream::once(future::ready(server_transport)))
        .respond_with(TestStorageEngine.serve());

    tokio::spawn(server);

    let mut client =
        StorageEngineClient::new(client::Config::default(), client_transport).spawn()?;

    let _session = client
        .use_database(
            context::current(),
            SessionWithoutDb::default(),
            DatabaseName::new("x").unwrap(),
        )
        .await?;

    Ok(())
}
