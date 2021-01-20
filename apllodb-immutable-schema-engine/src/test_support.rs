use apllodb_immutable_schema_engine_infra::external_interface::ApllodbImmutableSchemaEngine;
use apllodb_storage_engine_interface::StorageEngineClient;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread::JoinHandle,
};
use tokio::runtime::Builder;
use tokio_serde::formats::Bincode;
use portpicker::pick_unused_port;
use std::time::Duration;
use apllodb_shared_components::ApllodbResult;

/// Includes server address and JoinHandle
#[derive(Debug)]
pub struct TestServerHandler {
    pub join_handle: JoinHandle<()>,
    pub connect_addr: SocketAddr,
}

/// Spawn a thread to run ApllodbImmutableSchemaEngine server.
pub async fn spawn_server() -> ApllodbResult<TestServerHandler> {
    let port = pick_unused_port().expect("no TCP port available");
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    log::info!("starting ApllodbImmutableSchemaEngine server on {}...", socket);

    let rt = Builder::new_multi_thread().enable_all().build()?;
    let join_handle = std::thread::spawn(move || {
        let local = tokio::task::LocalSet::new();
        local.block_on(&rt, ApllodbImmutableSchemaEngine::serve(socket.clone())).unwrap();
    });

    async move {
        while let Err(_) = make_client(socket.clone()).await {
            log::debug!("waiting for server to start...");
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        log::info!("server has started");

        Ok(TestServerHandler {
            join_handle,
            connect_addr: socket,
        })
    }.await
}

pub async fn make_client(connect_addr: SocketAddr) -> ApllodbResult<StorageEngineClient> {
    let mut transport = tarpc::serde_transport::tcp::connect(connect_addr, Bincode::default);
    transport.config_mut().max_frame_length(4294967296);

    let client =
        StorageEngineClient::new(tarpc::client::Config::default(), transport.await?)
            .spawn()?;
    Ok(client)
}
