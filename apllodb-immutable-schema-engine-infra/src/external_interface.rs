use std::{
    cell::RefCell,
    net::{IpAddr, SocketAddr},
    pin::Pin,
    rc::Rc,
};

use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithDb, SessionWithoutDb};
use apllodb_storage_engine_interface::StorageEngine;
use futures::prelude::*;
use futures::{future::FutureExt, Future};
use tarpc::{
    context,
    server::{Channel, Handler},
};
use tokio_serde::formats::Bincode;

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::SqliteResourcePool};

type BoxFutResult<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>>>>;

/// Storage engine implementation.
#[derive(Clone, Debug)]
pub struct ApllodbImmutableSchemaEngine {
    addr: SocketAddr,

    // FIXME Consider sharding by SessionId to avoid writer contention using something like dashmap.
    // see: <https://tokio.rs/tokio/tutorial/shared-state#tasks-threads-and-contention>
    pool: Rc<RefCell<SqliteResourcePool<'static>>>,
}

impl ApllodbImmutableSchemaEngine {
    pub async fn serve(addr: SocketAddr) -> Result<(), std::io::Error> {
        let listener = tarpc::serde_transport::tcp::listen(&addr, Bincode::default).await?;

        listener
            // Ignore accept errors.
            .filter_map(|r| future::ready(r.ok()))
            .map(tarpc::server::BaseChannel::with_defaults)
            // Limit channels to 1 per IP.
            .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
            // serve is generated by the service attribute. It takes as input any type implementing
            // the generated World trait.
            .map(|channel| {
                println!("r");

                let server = Self {
                    addr: channel.as_ref().as_ref().peer_addr().unwrap(),
                    pool: Rc::new(RefCell::new(SqliteResourcePool::default())),
                };
                channel
                    .respond_with(server.serve())
                    .try_for_each(|request_handler| async move {
                        request_handler.await;
                        Ok(())
                    })
                    .map_ok(|()| log::info!("ClientHandler finished."))
                    .unwrap_or_else(|e| log::info!("ClientHandler errored out: {}", e))
            })
            // Max 10 channels.
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;

        Ok(())
    }
}

impl StorageEngine for ApllodbImmutableSchemaEngine {
    type UseDatabaseFut = BoxFutResult<SessionWithDb>;

    fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> Self::UseDatabaseFut {
        async move {
            let db = SqliteDatabase::use_database(database.clone())
                .await
                .unwrap();
            self.pool.borrow_mut().register_db(session.get_id(), db);

            Ok(session.upgrade(database))
        }
        .boxed_local()
    }
}
