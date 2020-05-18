mod constraint_kind;
mod constraints;
mod name;

pub use name::VersionSetName;

use apllo_shared_components::{
    data_structure::{ColumnDefinition, TableConstraints, TableName},
    error::AplloResult,
};
use constraints::VersionSetConstraints;
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
    name: VersionSetName,
    constraints: VersionSetConstraints,
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
    /// Create.
    ///
    /// # Failures
    ///
    /// - Errors from [VersionSetConstraints::new](foo.html).
    pub(crate) fn new(
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> AplloResult<Self> {
        let constraints = VersionSetConstraints::new(table_constraints, column_definitions)?;
        Ok(Self {
            name: VersionSetName::from(table_name.clone()),
            constraints,
        })
    }

    /// Ref to VersionSetName.
    ///
    /// Same as `T_create_table_command :: ... :: T_table_name`.
    pub fn name(&self) -> &VersionSetName {
        &self.name
    }
}
