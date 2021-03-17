//! Checks crash safety of `ApllodbServer::command` with arbitrary string input (most of them are invalid SQL).

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|s: String| {
    let session = Session::from(SessionWithoutDb::default());
    let _ = ApllodbServer::command(session, s).await;
});
