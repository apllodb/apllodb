#![deny(warnings, missing_debug_implementations)]

//! apllodb's server instance.
//!
//! Although serve should be bin crate and accept connections from clients in main loop,
//! I cannot find any good RPC server implementation to use with ApllodbImmutableSchemaEngine for now.
//! See: <https://github.com/darwin-education/apllodb/pull/67>
//!
//! This crate directly depends on apllodb-immutable-schema-engine currently, although apllodb's storage engine is designed to be plugable.
//! See <https://github.com/darwin-education/apllodb/issues/47#issuecomment-753779450> for future plan.
//!
//! # What's tested here?
//!
//! apllodb-server crate has a number of e2e tests in `tests/` directory.
//! All of them uses `tests/sql_test` module to write test scenarios with SQL.
//!
//! Here shows rules of file name and type of tests:
//!
//! | File name | Test type |
//! |--|--|
//! | `tests/simple_*.rs` | Human-readable tests to: <ul><li>quickly shows what type of SQLs are currently supported</li><li>proceed TDD</li></ul> |
//! | `tests/prop_*.rs` | Property-based tests to assure the system: <ul><li>doesn't crash</li><li>returns expected type of responses</li></ul> with any SQL inputs. |

#[macro_use]
extern crate derive_new;

mod apllodb_server;

pub use crate::apllodb_server::{
    response::success::{rec::Rec, rec_iter::RecIter, ApllodbCommandSuccess},
    ApllodbServer,
};

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

#[cfg(test)]
mod tests {
    use crate::test_support::test_setup;

    #[ctor::ctor]
    fn setup() {
        test_setup();
    }
}
