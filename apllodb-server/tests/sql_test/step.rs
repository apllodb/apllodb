use apllodb_shared_components::{ApllodbErrorKind, RecordIterator};

#[derive(Clone, PartialEq, Debug)]
pub struct Step {
    sql: &'static str,
    expected: StepRes,
}

impl Step {
    pub fn new(sql: &'static str, expected: StepRes) -> Self {
        Self { sql, expected }
    }

    pub(super) fn sql(&self) -> String {
        self.sql.to_string()
    }

    pub(super) fn expected(&self) -> StepRes {
        self.expected.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum StepRes {
    OkQuery(RecordIterator), // TODO レコードまるごとだと比較面倒かも。 |record| ... なクロージャーを取って、assert! を中で書かせるくらいが丁度いい？
    Ok,
    Err(ApllodbErrorKind),
}
