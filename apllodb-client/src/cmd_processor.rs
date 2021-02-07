use apllodb_server::{ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::{ApllodbResult, Session};

#[derive(Debug, new)]
pub(crate) struct CmdProcessor<'main> {
    server: &'main ApllodbServer,
}

impl<'main> CmdProcessor<'main> {
    pub(crate) async fn process(&self, session: Session, cmd: &str) -> ApllodbResult<Session> {
        let success = self.server.command(session, cmd.to_string()).await?;
        match success {
            ApllodbCommandSuccess::QueryResponse { session, records } => {
                let mut cnt = 0;

                for r in records {
                    cnt += 1;

                    let mut s = String::new();
                    for (ffr, value) in r.into_col_vals() {
                        s.push_str(&format!("{}: {}\t", ffr, value));
                    }
                    println!("{}", s);
                }

                println!("\n{} records in total\n", cnt);

                Ok(Session::from(session))
            }

            ApllodbCommandSuccess::ModificationResponse { session }
            | ApllodbCommandSuccess::DDLResponse { session }
            | ApllodbCommandSuccess::BeginTransactionResponse { session } => {
                Ok(Session::from(session))
            }

            ApllodbCommandSuccess::CreateDatabaseResponse { session } => Ok(session),

            ApllodbCommandSuccess::UseDatabaseResponse { session }
            | ApllodbCommandSuccess::TransactionEndResponse { session } => {
                Ok(Session::from(session))
            }
        }
    }
}
