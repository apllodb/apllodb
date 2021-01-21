mod controller;
mod server;

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use crate::sqlite::sqlite_resource_pool::SqliteResourcePool;

/// Storage engine implementation.
#[derive(Clone, Debug)]
pub struct ApllodbImmutableSchemaEngine {
    addr: SocketAddr,

    // FIXME Consider sharding by SessionId to avoid writer contention using something like dashmap.
    // see: <https://tokio.rs/tokio/tutorial/shared-state#tasks-threads-and-contention>
    pool: Arc<RwLock<SqliteResourcePool>>,
}
