use apllodb_server::{ApllodbServer, ApllodbSuccess};
use apllodb_shared_components::{ApllodbResult, Session};

#[derive(Debug, new)]
pub(crate) struct CmdProcessor<'main> {
    server: &'main ApllodbServer,
}

impl<'main> CmdProcessor<'main> {
    pub(crate) async fn process(&self, session: Session, cmd: &str) -> ApllodbResult<Session> {
        let success = self.server.command(session, cmd.to_string()).await?;
        match success {
            ApllodbSuccess::QueryResponse { session, records } => {
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

                Ok(Session::WithTx(session))
            }
            ApllodbSuccess::ModificationResponse { session }
            | ApllodbSuccess::DDLResponse { session } => {
                // log::warn!("automatically commits transaction for demo");
                // // TODO print "? rows affected"
                // server.commit_transaction(session).await?;
                Ok(Session::WithTx(session))
            }
            ApllodbSuccess::DatabaseResponse { session } => Ok(session),
        }
    }
}