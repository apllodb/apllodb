mod constraint_kind;
mod constraints;
mod id;
mod repository;

pub use constraints::TableWideConstraints;
pub use id::VTableId;
pub use repository::VTableRepository;

use crate::entity::Entity;
use apllodb_shared_components::{
    data_structure::{
        AlterTableAction, ColumnDefinition, DatabaseName, TableConstraints, TableName,
    },
    error::ApllodbResult,
};
use std::cmp::Ordering;

/// A version table, which has set of [Version](struct.Version.html)s.
///
/// A vtable is mutable. Its [TableWideConstraint](enum.TableWideConstraint.html)s are changed by
/// apllodb ALTER TABLE commands (ADDing / MODIFYing column with T_table_constraint, DROPping column).
///
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
#[derive(Clone, Eq, PartialEq, Hash, Debug, new)]
pub struct VTable {
    id: VTableId,
    table_wide_constraints: TableWideConstraints,
}

impl Entity for VTable {
    type Id = VTableId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
impl PartialOrd for VTable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl Ord for VTable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl VTable {
    /// Constructor called on CREATE TABLE.
    ///
    /// # Failures
    ///
    /// - Errors from [TableConstraints::new](foo.html).
    pub fn create(
        database_name: &DatabaseName,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<Self> {
        let constraints = TableWideConstraints::new(table_constraints, column_definitions)?;
        Ok(Self {
            id: VTableId::new(database_name, table_name),
            table_wide_constraints: constraints,
        })
    }

    pub fn alter(&mut self, _action: &AlterTableAction) -> ApllodbResult<()> {
        // TODO TableWideConstraints に影響のある操作だった場合に、自分自身を変更する

        Ok(())
    }

    /// Ref to TableName.
    ///
    /// Same as `T_create_table_command :: ... :: T_table_name`.
    pub fn table_name(&self) -> &TableName {
        &self.id.table_name
    }

    /// Ref to TableWideConstraints
    pub fn table_wide_constraints(&self) -> &TableWideConstraints {
        &self.table_wide_constraints
    }
}
