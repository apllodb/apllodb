#![deny(warnings, missing_debug_implementations)]

//! apllodb's client bin crate.

use std::io::BufRead;

use apllodb_server::{ApllodbServer, ApllodbSuccess};
use apllodb_shared_components::{ApllodbResult, DatabaseName, Session};
use clap::{App, Arg};

#[async_std::main]
async fn main() -> ApllodbResult<()> {
    env_logger::init();

    let flags = App::new("apllodb-client")
        .arg(
            Arg::with_name("db")
                .long("db")
                .value_name("STRING")
                .help("Database name to use.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let db = DatabaseName::new(flags.value_of("db").unwrap()).unwrap();

    let server = ApllodbServer::default();

    let stdin = std::io::stdin();

    eprint!("SQL> ");
    for line in stdin.lock().lines() {
        let sql = line?;

        let session = server.begin_transaction(db.clone()).await?;
        let session = Session::WithTx(session);

        let resp = server.command(session, sql.to_string()).await?;

        match resp {
            ApllodbSuccess::QueryResponse {
                session: _,
                records,
            } => {
                log::info!("query result: {:#?}", records);
            }
            ApllodbSuccess::ModificationResponse { session }
            | ApllodbSuccess::DDLResponse { session } => {
                log::warn!("automatically commits transaction for demo");
                server.commit_transaction(session).await?;
            }
        };

        eprint!("SQL> ");
    }

    Ok(())
}
