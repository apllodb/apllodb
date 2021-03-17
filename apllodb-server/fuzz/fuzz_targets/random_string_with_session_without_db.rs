//! Checks crash safety of `ApllodbServer::command` with arbitrary string input (most of them are invalid SQL).
//!
//! Note that test inputs here rarely pass even lexer. Almost none of them passes parser.
//! To increase test coverage, an SQL fuzzer is required.
//!
//! Related articles:
//!
//! - <https://www.cockroachlabs.com/blog/sqlsmith-randomized-sql-testing/>

#![no_main]

use apllodb_server::{ApllodbServer, Session};
use async_std::task::block_on;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: String| {
    let session = Session::default();
    let server = ApllodbServer::default();

    block_on(async move {
        let _ = server.command(session, s).await;
    })
});
