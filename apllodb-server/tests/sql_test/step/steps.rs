use super::{Step, StepRes};

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Steps {
    BeginTransaction,
    CreateTablePeople,
}

impl From<Steps> for Vec<Step> {
    fn from(steps: Steps) -> Self {
        match steps {
            Steps::BeginTransaction => {
                vec![Step::new("BEGIN", StepRes::Ok)]
            }
            Steps::CreateTablePeople => {
                let mut steps = Self::from(Steps::BeginTransaction);
                steps.push(Step::new("CREATE TABLE people (id INTEGER NOT NULL, age INTEGER NOT NULL, PRIMARY KEY (id))", StepRes::Ok));
                steps
            }
        }
    }
}
