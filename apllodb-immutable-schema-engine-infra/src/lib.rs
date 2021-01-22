//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

#[macro_use]
extern crate derive_new;

mod engine;

mod access_methods;
mod error;
mod immutable_schema_row_iter;
mod sqlite;

#[cfg(test)]
mod test_support;

pub use crate::{
    engine::ApllodbImmutableSchemaEngine,
    sqlite::sqlite_resource_pool::db_pool::SqliteDatabasePool,
    sqlite::sqlite_resource_pool::tx_pool::SqliteTxPool,
};

// TODO remove after infra layer interface becomes async
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
