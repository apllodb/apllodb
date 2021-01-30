mod step;

pub use self::step::{step_res::StepRes, steps::Steps, Step};
use apllodb_server::ApllodbServer;
use apllodb_shared_components::{Session, SessionWithoutDb};

#[derive(Debug, Default)]
pub struct SqlTest {
    server: ApllodbServer,
    steps: Vec<Step>,
}

impl SqlTest {
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

    pub async fn run(self) {
        let mut cur_session = Session::from(SessionWithoutDb::default());
        for step in &self.steps {
            cur_session = step.run(&self.server, cur_session).await;
        }
    }
}
