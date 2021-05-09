use std::fmt::Debug;

use apllodb_server::{SqlState, ApllodbResult, Records};

#[allow(dead_code)]
pub enum StepRes {
    OkQuery(Box<dyn Fn(Records) -> ApllodbResult<()>>),
    Ok,
    Err(SqlState),
}

impl Debug for StepRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepRes::OkQuery(_) => write!(f, "StepRes::OkQuery(...)"),
            StepRes::Ok => write!(f, "StepRes::Ok"),
            StepRes::Err(e) => write!(f, "StepRes::Err({:?})", e),
        }
    }
}
