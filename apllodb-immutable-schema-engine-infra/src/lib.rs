//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

pub mod external_interface;

mod access_methods;
mod db_repo;
mod immutable_schema_row_iter;
mod sqlite;
mod tx_repo;

#[cfg(test)]
mod test_support;
