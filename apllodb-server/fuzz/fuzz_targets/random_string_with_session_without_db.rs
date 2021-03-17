//! Checks crash safety of `ApllodbServer::command` with arbitrary string input (most of them are invalid SQL).

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
