use apllodb_server::ApllodbServer;
use apllodb_shared_components::{Session, SessionWithoutDb};

use super::{Step, Steps};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SessionAB {
    A,
    B,
}

#[derive(Debug, Default)]
pub struct SqlTestSessionAB {
    server: ApllodbServer,
    steps: Vec<(Step, SessionAB)>,
}

impl SqlTestSessionAB {
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
