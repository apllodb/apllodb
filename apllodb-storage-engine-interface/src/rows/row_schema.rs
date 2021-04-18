use std::collections::HashSet;

use apllodb_shared_components::{RPos, Schema};

use crate::table_column_name::TableColumnName;

use sorted_vec::SortedSet;

/// Schema of [Row](crate::Row)s holding pairs of (RowPos, TableColumnName).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RowSchema(SortedSet<TableColumnName>);

impl Schema for RowSchema {
    type Name = TableColumnName;

    fn new(names: HashSet<TableColumnName>, _: usize) -> Self
    where
        Self: Sized,
    {
        let inner = SortedSet::from_unsorted(names.into_iter().collect());
        Self(inner)
    }

    fn names_with_pos(&self) -> Vec<(RPos, Option<TableColumnName>)> {
        self.0
            .iter()
            .enumerate()
            .map(|(raw_pos, tn)| (RPos::new(raw_pos), Some(tn.clone())))
            .collect()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl RowSchema {
    pub fn empty() -> Self {
        Self(SortedSet::new())
    }

    pub fn table_column_names(&self) -> Vec<TableColumnName> {
        self.0.iter().cloned().collect()
    }

    pub fn table_column_names_with_pos(&self) -> Vec<(RPos, TableColumnName)> {
        self.table_column_names()
            .into_iter()
            .enumerate()
            .map(|(raw_pos, tc)| (RPos::new(raw_pos), tc))
            .collect()
    }
}

impl From<HashSet<TableColumnName>> for RowSchema {
    fn from(names: HashSet<TableColumnName>) -> Self {
        Self::new(names, 0)
    }
}
