use apllodb_shared_components::{RPos, Schema};
use serde::{Deserialize, Serialize};

use crate::table_column_name::TableColumnName;

/// Schema of [Row](crate::Row)s holding pairs of (RowPos, TableColumnName).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RowSchema {
    inner: Vec<(RPos, TableColumnName)>,
}

impl Schema for RowSchema {
    type Name = TableColumnName;

    fn new(names_with_pos: Vec<(RPos, Option<TableColumnName>)>) -> Self
    where
        Self: Sized,
    {
        let inner = names_with_pos
            .into_iter()
            .map(|(pos, opt_tc)| {
                let tc = opt_tc.expect("All parts in RowSchema must have TableColumnName");
                (pos, tc)
            })
            .collect();
        Self { inner }
    }

    fn names_with_pos(&self) -> Vec<(RPos, Option<TableColumnName>)> {
        self.inner
            .iter()
            .map(|(pos, tn)| (*pos, Some(tn.clone())))
            .collect()
    }
}

impl From<Vec<TableColumnName>> for RowSchema {
    fn from(names: Vec<TableColumnName>) -> Self {
        Self {
            inner: names
                .into_iter()
                .enumerate()
                .map(|(raw_pos, name)| (RPos::new(raw_pos), name))
                .collect(),
        }
    }
}
