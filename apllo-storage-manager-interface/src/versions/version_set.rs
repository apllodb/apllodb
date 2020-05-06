mod column_definition;

use apllo_shared_components::{ColumnDefinition, TableConstraint, TableName};
use column_definition::VersionSetColumnDefinition;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VersionSet {
    name: String,
    column_definitions: Vec<VersionSetColumnDefinition>,
}

impl Ord for VersionSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for VersionSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl VersionSet {
    pub(crate) fn new(
        _table_name: &TableName,
        _table_constraints: &[TableConstraint],
        _column_definitions: &[ColumnDefinition],
    ) -> Self {
        todo!()
    }
}
