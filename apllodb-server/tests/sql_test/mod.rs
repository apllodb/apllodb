mod step;

pub use self::step::{steps::Steps, Step, StepRes};
use apllodb_server::{ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::{Session, SessionWithoutDb};
use pretty_assertions::assert_eq;

#[derive(Debug, Default)]
pub struct SqlTest {
    server: ApllodbServer,
    steps: Vec<Step>,
}

impl SqlTest {
    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }

    pub fn add_steps(&mut self, steps: Steps) {
        let steps: Vec<Step> = steps.into();
        for step in steps {
            self.add_step(step);
        }
    }

    pub async fn run(&mut self) {
        let mut cur_session = Session::from(SessionWithoutDb::default());

        for step in &self.steps {
            match self.server.command(cur_session, step.sql()).await {
                Ok(success) => match success {
                    ApllodbCommandSuccess::QueryResponse {
                        session: sess,
                        records,
                    } => {
                        cur_session = Session::from(sess);
                        match step.expected() {
                            StepRes::OkQuery(expected_records) => {
                                assert_eq!(expected_records, records);
                            }

                            StepRes::Ok => {
                                panic!(
                                    "use StepRes::OkQuery for Step with SELECT SQL - step: {:#?}",
                                    step
                                )
                            }
                            StepRes::Err(_) => {
                                panic!("SELECT SQL has unexpectedly succeeded - step: {:#?}", step)
                            }
                        }
                    }

                    ApllodbCommandSuccess::ModificationResponse { session }
                    | ApllodbCommandSuccess::DDLResponse { session }
                    | ApllodbCommandSuccess::BeginTransactionResponse { session } => {
                        cur_session = Session::from(session);
                        match step.expected() {
                            StepRes::Ok => {}

                            StepRes::OkQuery(_) => {
                                panic!(
                                    "StepRes::OkQuery is only for SELECT SQL - step: {:#?}",
                                    step
                                )
                            }
                            StepRes::Err(_) => {
                                panic!("SQL has unexpectedly succeeded - step: {:#?}", step)
                            }
                        }
                    }

                    ApllodbCommandSuccess::CreateDatabaseResponse { session } => {
                        cur_session = session;
                    }

                    ApllodbCommandSuccess::UseDatabaseResponse { session }
                    | ApllodbCommandSuccess::TransactionEndResponse { session } => {
                        cur_session = Session::from(session);
                    }
                },
                Err(e) => {
                    cur_session = e.session;
                    let e = e.err;

                    match step.expected() {
                        StepRes::Err(kind) => {
                            assert_eq!(&kind, e.kind());
                        }
                        _ => panic!(
                            "unexpected error {:#?} on ApllodbServer::command() - step: {:#?}",
                            e, step
                        ),
                    }
                }
            }
        }
    }
}
