#![deny(missing_debug_implementations)]

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
//! apllodb-server crate has a number of E2E tests in `tests/` directory.
//!
//! Here shows rules of file name and type of tests:
//!
//! | File name | Test type |
//! |--|--|
//! | `tests/func_*.rs` | Functional tests to: <ul><li>quickly shows what type of SQLs are currently supported</li><li>tests with heuristic edge cases</li><li>proceed TDD</li></ul> |
//! | `tests/scenario_*.rs` | Scenario tests. |

#[macro_use]
extern crate derive_new;

mod apllodb_server;

// re-export from apllodb-shared-components
pub use crate::apllodb_server::{response::success::ApllodbCommandSuccess, ApllodbServer};
pub use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionResult, SchemaIndex, Session, SqlState,
};
pub use apllodb_sql_processor::{Record, RecordIndex, Records};

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
