use apllodb_server::ApllodbServer;
use apllodb_shared_components::{DatabaseName, Session};

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
        let database_name = DatabaseName::random();
        let mut session_a =
            Session::from(session_with_db(&self.server, database_name.clone()).await);
        let mut session_b = Session::from(session_with_db(&self.server, database_name).await);

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
