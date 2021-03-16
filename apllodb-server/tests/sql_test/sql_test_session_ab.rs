use apllodb_immutable_schema_engine_infra::test_support::sqlite_database_cleaner::SqliteDatabaseCleaner;
use apllodb_server::ApllodbServer;
use apllodb_shared_components::{DatabaseName, Session, SessionWithoutDb};
use futures::FutureExt;

use super::{session_with_db, Step, Steps};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SessionAB {
    A,
    B,
}

#[derive(Debug)]
pub struct SqlTestSessionAB {
    server: ApllodbServer,
    steps: Vec<(Step, SessionAB)>,
    database_name: DatabaseName,
}

impl Default for SqlTestSessionAB {
    fn default() -> Self {
        Self {
            server: ApllodbServer::default(),
            steps: vec![],
            database_name: DatabaseName::random(),
        }
    }
}

impl SqlTestSessionAB {
    /// The order of `add_step()` call represents "The order of SQL commands *issued*", not "the order of SQL commands *finished*".
    ///
    /// ```text
    /// .add_step(SessionAB::A, sql_a1)
    /// .add_step(SessionAB::A, sql_a2)
    /// .add_step(SessionAB::B, sql_b1)
    /// .add_step(SessionAB::A, sql_a3)
    /// ```
    ///
    /// The order of SQL commands issued:
    ///
    /// 1. sql_a1
    /// 2. sql_a2
    /// 3. sql_b1  // sql_a2 might not have finished
    /// 4. sql_a3  // sql_a2 has finished; sql_b1 might not have finished
    ///
    /// NOTE: do not pass database command like "CREATE DATABASE" / "USE DATABASE" / ...
    /// Database is automatically created / used in run().
    pub fn add_step(mut self, session_ab: SessionAB, step: Step) -> Self {
        self.steps.push((step, session_ab));
        self
    }

    #[allow(dead_code)] // seemingly every tests/*.rs must call this func not to be `dead_code`
    pub fn add_steps(mut self, session_ab: SessionAB, steps: Steps) -> Self {
        let steps: Vec<Step> = steps.into();
        for step in steps {
            self = self.add_step(session_ab.clone(), step);
        }
        self
    }

    #[allow(dead_code)]
    pub async fn run(self) {
        let server_a = self.server.clone();
        let server_b = self.server.clone();

        let database_name_a = DatabaseName::random();
        let database_name_b = database_name_a.clone();

        let _db_cleaner = SqliteDatabaseCleaner::new(database_name_a.clone());

        let mut session_a_fut =
            async move { Session::from(session_with_db(&server_a, database_name_a).await) }
                .boxed_local();
        let mut session_b_fut =
            async move { Session::from(session_with_db(&server_b, database_name_b).await) }
                .boxed_local();

        for (step, session_ab) in &self.steps {
            match session_ab {
                SessionAB::A => {
                    let session_a = session_a_fut.await;
                    session_a_fut = step.run(&self.server, session_a).boxed_local();
                }
                SessionAB::B => {
                    let session_b = session_b_fut.await;
                    session_b_fut = step.run(&self.server, session_b).boxed_local();
                }
            }
        }

        let _ = session_a_fut.await;
        let _ = session_b_fut.await;
    }

    #[allow(dead_code)]
    pub async fn run_with_manual_db_control(self) {
        let mut session_a = Session::from(SessionWithoutDb::default());
        let mut session_b = Session::from(SessionWithoutDb::default());

        for (step, session_ab) in &self.steps {
            match session_ab {
                SessionAB::A => {
                    session_a = step.run(&self.server, session_a).await;
                }
                SessionAB::B => {
                    session_b = step.run(&self.server, session_b).await;
                }
            }
        }
    }
}
