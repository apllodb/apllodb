#![deny(warnings, missing_debug_implementations)]

//! apllodb's server instance.
//!
//! Although serve should be bin crate and accept connections from clients in main loop,
//! I cannot find any good RPC server implementation to use with ApllodbImmutableSchemaEngine for now.
//! See: <https://github.com/darwin-education/apllodb/pull/67>
//!
//! This crate directly depends on apllodb-immutable-schema-engine currently, although apllodb's storage engine is designed to be plugable.
//! See <https://github.com/darwin-education/apllodb/issues/47#issuecomment-753779450> for future plan.

#[macro_use]
extern crate derive_new;

mod apllodb_server;

pub use crate::apllodb_server::ApllodbServer;
