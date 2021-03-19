#![deny(warnings, missing_debug_implementations)]

//! apllodb's CLI client.

#[macro_use]
extern crate derive_new;

mod cmd_processor;
mod shell;

use apllodb_server::{ApllodbServer, Session};
use cmd_processor::CmdProcessor;
use rustyline::error::ReadlineError;
use shell::ReadLine;

#[async_std::main]
async fn main() {
    env_logger::init();

    let server = ApllodbServer::default();
    let mut rl = ReadLine::default();
    let cmd_processor = CmdProcessor::new(&server);

    let mut session = Session::default();

    loop {
        match rl.readline() {
            Ok(cmd) => {
                session = match cmd_processor.process(session, &cmd).await {
                    Ok(sess) => {
                        let _ = rl.add_history(&cmd);
                        sess
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                        e.session
                    }
                }
            }

            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                log::error!("{:?}", e);
                continue;
            }
        }
    }
}
