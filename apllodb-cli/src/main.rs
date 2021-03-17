#![deny(warnings, missing_debug_implementations)]

//! apllodb's CLI client.

#[macro_use]
extern crate derive_new;

mod cmd_processor;
mod shell;

use apllodb_server::{ApllodbResult, ApllodbServer, Session};
use cmd_processor::CmdProcessor;
use rustyline::error::ReadlineError;
use shell::ReadLine;

#[async_std::main]
async fn main() -> ApllodbResult<()> {
    env_logger::init();

    let server = ApllodbServer::default();
    let mut rl = ReadLine::default();
    let cmd_processor = CmdProcessor::new(&server);

    let mut session = Session::default();

    loop {
        match rl.readline() {
            Ok(cmd) => {
                session = cmd_processor.process(session, &cmd).await?;
                let _ = rl.add_history(&cmd);
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
