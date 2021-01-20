use apllodb_shared_components::{
    ApllodbResult, DatabaseName, SessionWithoutDb, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{
    test_support::TestStorageEngineImpl, StorageEngine, StorageEngineClient,
};
use futures::{future, prelude::*};
use tarpc::{
    client, context,
    server::{self, Handler},
};

#[tokio::test]
async fn test_in_process_client() -> ApllodbResult<()> {
    let (client_transport, server_transport) = tarpc::transport::channel::unbounded();

    let server = server::new(server::Config::default())
        .incoming(stream::once(future::ready(server_transport)))
        .respond_with(TestStorageEngineImpl.serve());

    tokio::spawn(server);

    let mut client =
        StorageEngineClient::new(client::Config::default(), client_transport).spawn()?;

    let _session = client
        .use_database(
            context::current(),
            SessionWithoutDb::default(),
            DatabaseName::new("x")?,
        )
        .await??;

    // let session = client
    //     .begin_transaction(context::current(), session)
    //     .await??;

    // let session = client
    //     .create_table(
    //         context::current(),
    //         session,
    //         TableName::new("t")?,
    //         TableConstraints::default(),
    //         vec![],
    //     )
    //     .await??;

    // client
    //     .commit_transaction(context::current(), session)
    //     .await??;

    Ok(())
}
