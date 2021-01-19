//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

pub mod external_interface;

mod access_methods;
mod immutable_schema_row_iter;
mod sqlite;

#[cfg(test)]
mod test_support;

// TODO remove after infra layer interface becomes async
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
