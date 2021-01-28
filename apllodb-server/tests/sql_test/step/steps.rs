use apllodb_shared_components::DatabaseName;

use super::{Step, StepRes};

#[derive(Clone, PartialEq, Debug)]
pub enum Steps {
    UseDatabase,
}

impl From<Steps> for Vec<Step> {
    fn from(steps: Steps) -> Self {
        match steps {
            Steps::UseDatabase => {
                let database_name = DatabaseName::random();
                vec![
                    Step::new(
                        format!("CREATE DATABASE {}", database_name.as_str()),
                        StepRes::Ok,
                    ),
                    Step::new(
                        format!("USE DATABASE {}", database_name.as_str()),
                        StepRes::Ok,
                    ),
                ]
            }
        }
    }
}
