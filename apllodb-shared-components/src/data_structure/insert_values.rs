use serde::{Deserialize, Serialize};

use crate::{Record, SqlValue};

/// Values for INSERT command.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct InsertValues(Vec<SqlValue>);

/// used for `INSERT INTO t (a, b, c) SELECT x, y, z FROM s;`.
impl From<Record> for InsertValues {
    fn from(_r: Record) -> Self {
        todo!()
    }
}
