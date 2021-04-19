mod record_cli_display;

use apllodb_server::{ApllodbCommandSuccess, ApllodbServer, ApllodbSessionResult, Session};
use record_cli_display::RecordCliDisplay;

#[derive(Debug, new)]
pub(crate) struct CmdProcessor<'main> {
    server: &'main ApllodbServer,
}

impl<'main> CmdProcessor<'main> {
    pub(crate) async fn process(
        &self,
        session: Session,
        cmd: &str,
    ) -> ApllodbSessionResult<Session> {
        let success = self.server.command(session, cmd.to_string()).await?;
        match success {
            ApllodbCommandSuccess::QueryResponse { session, records } => {
                let mut cnt = 0;

                for r in records {
                    cnt += 1;
                    println!("{}", r.cli_display());
                }

                println!("\n{} records in total\n", cnt);

                Ok(Session::from(session))
            }

            ApllodbCommandSuccess::ModificationResponse { session }
            | ApllodbCommandSuccess::DdlResponse { session }
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
