use apllodb_server::{ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::Session;

use self::step_res::StepRes;

pub(crate) mod step_res;
pub(crate) mod steps;

#[derive(Clone, PartialEq, Debug)]
pub struct Step {
    sql: String,
    expected: StepRes,
}

impl Step {
    pub fn new(sql: impl Into<String>, expected: StepRes) -> Self {
        Self {
            sql: sql.into(),
            expected,
        }
    }

    pub(super) async fn run(&self, server: &ApllodbServer, session: Session) -> Session {
        match server.command(session, self.sql.to_string()).await {
            Ok(success) => match success {
                ApllodbCommandSuccess::QueryResponse {
                    session: sess,
                    records,
                } => {
                    match &self.expected {
                        StepRes::OkQuery(expected_records) => {
                            assert_eq!(expected_records, &records);
                        }

                        StepRes::Ok => {
                            panic!(
                                "use StepRes::OkQuery for Step with SELECT SQL - step: {:#?}",
                                self
                            )
                        }
                        StepRes::Err(_) => {
                            panic!("SELECT SQL has unexpectedly succeeded - step: {:#?}", self)
                        }
                    }
                    Session::from(sess)
                }

                ApllodbCommandSuccess::ModificationResponse { session }
                | ApllodbCommandSuccess::DDLResponse { session }
                | ApllodbCommandSuccess::BeginTransactionResponse { session } => {
                    match &self.expected {
                        StepRes::Ok => {}

                        StepRes::OkQuery(_) => {
                            panic!(
                                "StepRes::OkQuery is only for SELECT SQL - step: {:#?}",
                                self
                            )
                        }
                        StepRes::Err(_) => {
                            panic!("SQL has unexpectedly succeeded - step: {:#?}", self)
                        }
                    }
                    Session::from(session)
                }

                ApllodbCommandSuccess::CreateDatabaseResponse { session } => session,

                ApllodbCommandSuccess::UseDatabaseResponse { session }
                | ApllodbCommandSuccess::TransactionEndResponse { session } => {
                    Session::from(session)
                }
            },
            Err(sess_err) => {
                let e = sess_err.err;

                match &self.expected {
                    StepRes::Err(kind) => {
                        assert_eq!(kind, e.kind());
                    }
                    _ => panic!(
                        "unexpected error {:#?} on ApllodbServer::command() - step: {:#?}",
                        e, self
                    ),
                }
                sess_err.session
            }
        }
    }
}
