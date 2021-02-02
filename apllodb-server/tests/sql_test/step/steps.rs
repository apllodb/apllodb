use super::{Step, StepRes};

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Steps {
    BeginTransaction,
    CreateTablePeople,
    CreateTableBody,
    CreateTablePet,
    SetupPeopleDataset,
    SetupBodyDataset,
    SetupPetDataset,
    SetupPeopleBodyPetDataset,
}

impl From<Steps> for Vec<Step> {
    fn from(steps: Steps) -> Self {
        match steps {
            Steps::BeginTransaction => {
                vec![Step::new("BEGIN", StepRes::Ok)]
            }
            Steps::CreateTablePeople => {
                let mut steps = Self::from(Steps::BeginTransaction);
                steps.push(Step::new("CREATE TABLE people (id BIGINT NOT NULL, age INTEGER NOT NULL, PRIMARY KEY (id))", StepRes::Ok));
                steps.push(Step::new("COMMIT", StepRes::Ok));
                steps
            }
            Steps::CreateTableBody => {
                let mut steps = Self::from(Steps::BeginTransaction);
                steps.push(Step::new("CREATE TABLE body (id BIGINT NOT NULL, people_id BIGINT NOT NULL, height INTEGER NOT NULL, PRIMARY KEY (id))", StepRes::Ok));
                steps.push(Step::new("COMMIT", StepRes::Ok));
                steps
            }
            Steps::CreateTablePet => {
                let mut steps = Self::from(Steps::BeginTransaction);
                steps.push(Step::new("CREATE TABLE pet (id BIGINT NOT NULL, people_id BIGINT NOT NULL, kind TEXT NOT NULL, age SMALLINT NOT NULL, PRIMARY KEY (id))", StepRes::Ok));
                steps.push(Step::new("COMMIT", StepRes::Ok));
                steps
            }
            Steps::SetupPeopleDataset => {
                let mut steps = Self::from(Steps::CreateTablePeople);
                steps.push(Step::new("BEGIN", StepRes::Ok));
                steps.push(Step::new(
                    "INSERT INTO people (id, age) VALUES (1, 13)",
                    StepRes::Ok,
                ));
                steps.push(Step::new(
                    "INSERT INTO people (id, age) VALUES (2, 70)",
                    StepRes::Ok,
                ));
                steps.push(Step::new(
                    "INSERT INTO people (id, age) VALUES (3, 35)",
                    StepRes::Ok,
                ));
                steps.push(Step::new("COMMIT", StepRes::Ok));
                steps
            }
            Steps::SetupBodyDataset => {
                let mut steps = Self::from(Steps::CreateTableBody);
                steps.push(Step::new("BEGIN", StepRes::Ok));
                steps.push(Step::new(
                    "INSERT INTO body (id, people_id, height) VALUES (1, 1, 145)",
                    StepRes::Ok,
                ));
                steps.push(Step::new(
                    "INSERT INTO body (id, people_id, height) VALUES (2, 3, 175)",
                    StepRes::Ok,
                ));
                steps.push(Step::new("COMMIT", StepRes::Ok));
                steps
            }
            Steps::SetupPetDataset => {
                let mut steps = Self::from(Steps::CreateTablePet);
                steps.push(Step::new("BEGIN", StepRes::Ok));
                steps.push(Step::new(
                    r#"INSERT INTO pet (id, people_id, kind, age) VALUES (1, 1, "dog", 13)"#,
                    StepRes::Ok,
                ));
                steps.push(Step::new(
                    r#"INSERT INTO pet (id, people_id, kind, age) VALUES (2, 3, "dog", 5)"#,
                    StepRes::Ok,
                ));
                steps.push(Step::new(
                    r#"INSERT INTO pet (id, people_id, kind, age) VALUES (3, 3, "cat", 3)"#,
                    StepRes::Ok,
                ));
                steps.push(Step::new("COMMIT", StepRes::Ok));
                steps
            }
            Steps::SetupPeopleBodyPetDataset => {
                let mut steps = Self::from(Steps::SetupPeopleDataset);
                steps.append(&mut Self::from(Steps::SetupBodyDataset));
                steps.append(&mut Self::from(Steps::SetupPetDataset));
                steps
            }
        }
    }
}
