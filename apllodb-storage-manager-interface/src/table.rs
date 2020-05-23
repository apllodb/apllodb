mod constraint_kind;
mod constraints;

use apllodb_shared_components::{
    data_structure::{ColumnDefinition, TableConstraints, TableName},
    error::ApllodbResult,
};
use constraints::TableWideConstraints;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Set of [Version](struct.Version.html)s.
///
/// A Table is mutable. Its [TableConstraint](enum.TableConstraint.html)s are changed by
/// apllodb ALTER TABLE commands (ADDing / MODIFYing column with T_table_constraint, DROPping column).
///
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Table {
    name: TableName,
    constraints: TableWideConstraints,
}

impl Ord for Table {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Table {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Table {
    /// Create.
    ///
    /// # Failures
    ///
    /// - Errors from [TableConstraints::new](foo.html).
    pub(crate) fn new(
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<Self> {
        let constraints = TableWideConstraints::new(table_constraints, column_definitions)?;
        Ok(Self {
            name: TableName::from(table_name.clone()),
            constraints,
        })
    }

    /// Ref to TableName.
    ///
    /// Same as `T_create_table_command :: ... :: T_table_name`.
    pub fn name(&self) -> &TableName {
        &self.name
    }
}
