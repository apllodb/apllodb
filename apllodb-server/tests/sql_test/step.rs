use self::step_res::StepRes;

pub(crate) mod step_res;
pub(crate) mod steps;

#[derive(Clone, PartialEq, Debug)]
pub struct Step {
    sql: String,
    expected: StepRes,
}

impl Step {
    pub fn new(sql: impl Into<String>, expected: StepRes) -> Self {
        Self {
            sql: sql.into(),
            expected,
        }
    }

    pub(super) fn sql(&self) -> String {
        self.sql.to_string()
    }

    pub(super) fn expected(&self) -> StepRes {
        self.expected.clone()
    }
}
