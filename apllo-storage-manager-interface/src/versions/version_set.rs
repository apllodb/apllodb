mod constraint;

use apllo_shared_components::{ColumnDefinition, TableConstraint, TableName};
use constraint::VersionSetConstraint;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Set of [Version](struct.Version.html)s.
///
/// A VersionSet is mutable. Its [VersionSetConstraint](enum.VersionSetConstraint.html)s are changed by
/// APLLO ALTER TABLE commands (ADDing / MODIFYing column with T_table_constraint, DROPping column).
///
/// See: https://github.com/darwin-education/apllo/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct VersionSet {
    name: String,
    constraints: Vec<VersionSetConstraint>,
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

    /// Name of VersionSet.
    ///
    /// Same as `T_create_table_command :: ... :: T_table_name`.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
