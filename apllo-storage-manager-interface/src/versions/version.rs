mod column_definition;

use apllo_shared_components::ColumnDefinition;

use column_definition::VersionColumnDefinition;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Version {
    number: u64,
    column_definitions: Vec<VersionColumnDefinition>,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Version {
    pub(crate) fn new(_column_definitions: &[ColumnDefinition]) -> Self {
        todo!()
    }
}
