mod step;

use apllodb_server::ApllodbServer;
use apllodb_shared_components::{Session, SessionWithoutDb};

use self::step::Step;

#[derive(Debug, Default)]
pub struct SqlTest {
    server: ApllodbServer,

    steps: Vec<Step>,
}

impl SqlTest {
    pub async fn run(&self) {
        let mut session = Session::from(SessionWithoutDb::default());

        for step in self.steps {
            let cmd_result = self.server.command(session, step.sql()).await;
            // TODO Err 帰ってきたときでも以前のsessionを返してくれなきゃ困る
        }
    }
}
