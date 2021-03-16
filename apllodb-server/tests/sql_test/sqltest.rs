use apllodb_immutable_schema_engine_infra::test_support::sqlite_database_cleaner::SqliteDatabaseCleaner;
use apllodb_server::ApllodbServer;
use apllodb_shared_components::{DatabaseName, Session, SessionWithoutDb};

use super::{session_with_db, Step, Steps};

#[derive(Debug, Default)]
pub struct SqlTest {
    server: ApllodbServer,
    steps: Vec<Step>,
}

impl SqlTest {
    /// NOTE: do not pass database command like "CREATE DATABASE" / "USE DATABASE" / ...
    /// Database is automatically created / used in run().
    pub fn add_step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    #[allow(dead_code)] // seemingly every tests/*.rs must call this func not to be `dead_code`
    pub fn add_steps(mut self, steps: Steps) -> Self {
        let steps: Vec<Step> = steps.into();
        for step in steps {
            self = self.add_step(step);
        }
        self
    }

    #[allow(dead_code)]
    pub async fn run(self) {
        let db_name = DatabaseName::random();
        let _db_cleaner = SqliteDatabaseCleaner::new(db_name.clone());

        let mut cur_session = Session::from(session_with_db(&self.server, db_name).await);
        for step in &self.steps {
            cur_session = step.run(&self.server, cur_session).await;
        }
    }

    #[allow(dead_code)]
    pub async fn run_with_manual_db_control(self) {
        let mut cur_session = Session::from(SessionWithoutDb::default());
        for step in &self.steps {
            cur_session = step.run(&self.server, cur_session).await;
        }
    }
}
