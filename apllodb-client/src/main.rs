#![deny(warnings, missing_debug_implementations)]

//! apllodb's client bin crate.

use apllodb_server::{ApllodbServer, ApllodbSuccess};
use apllodb_shared_components::{ApllodbResult, DatabaseName, Session};
use clap::{App, Arg};
use rustyline::{error::ReadlineError, Editor};

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

    let history_path = {
        let mut p = dirs::home_dir().expect("cannot detect HOME directory to put history file");
        p.push(".apllodb_history");
        p
    };

    let mut rl = Editor::<()>::new(); // TODO SQL completion
    let _ = rl.load_history(&history_path);

    loop {
        match rl.readline("SQL> ") {
            Ok(sql) => {
                let session = server.begin_transaction(db.clone()).await?;
                let session = Session::WithTx(session);

                let success = server.command(session, sql.clone()).await?;

                rl.add_history_entry(sql.as_str());
                rl.save_history(&history_path).unwrap();

                match success {
                    ApllodbSuccess::QueryResponse {
                        session: _,
                        records,
                    } => {
                        let mut cnt = 0;

                        for r in records {
                            cnt += 1;

                            let mut s = String::new();
                            // TODO use field order in query
                            for (field, value) in r.into_field_values() {
                                s.push_str(&format!("{}: {}\t", field, value));
                            }
                            println!("{}", s);
                        }

                        println!("\n{} records in total\n", cnt);
                    }
                    ApllodbSuccess::ModificationResponse { session }
                    | ApllodbSuccess::DDLResponse { session } => {
                        log::warn!("automatically commits transaction for demo");
                        // TODO print "? rows affected"
                        server.commit_transaction(session).await?;
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
