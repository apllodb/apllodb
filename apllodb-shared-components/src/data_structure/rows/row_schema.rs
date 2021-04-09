use crate::{RPos, Schema, TableColumnName};
use serde::{Deserialize, Serialize};

/// Schema of [Row](crate::Row)s holding pairs of (RowPos, TableColumnName).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RowSchema {
    inner: Vec<(RPos, TableColumnName)>,
}

impl Schema for RowSchema {
    type Name = TableColumnName;

    fn names_with_pos(&self) -> Vec<(RPos, Option<TableColumnName>)> {
        self.inner
            .iter()
            .map(|(pos, tn)| (*pos, Some(tn.clone())))
            .collect()
    }
}
