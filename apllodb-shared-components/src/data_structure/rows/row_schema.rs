use crate::{RPos, Schema, TableColumnName};
use serde::{Deserialize, Serialize};

use super::row_index::RowIndex;

/// Schema of [Row](crate::Row)s holding pairs of (RowPos, TableColumnName).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RowSchema {
    inner: Vec<(RPos, TableColumnName)>,
}

impl Schema for RowSchema {
    type Name = TableColumnName;

    type Index = RowIndex;

    fn names_with_pos(&self) -> &[(RPos, Self::Name)] {
        &self.inner
    }
}
