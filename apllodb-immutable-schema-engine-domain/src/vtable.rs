mod constraint_kind;
mod constraints;
mod id;
mod version_repo;

use crate::{entity::Entity, ActiveVersion, ImmutableSchemaTx};
use apllodb_shared_components::{
    data_structure::{AlterTableAction, ColumnDefinition, ColumnName, TableConstraints, TableName},
    error::ApllodbResult,
};
use constraints::TableWideConstraints;
use id::VTableId;
use std::cmp::Ordering;
use version_repo::VersionRepo;

/// A version table, which has set of [Version](struct.Version.html)s.
///
/// A vtable is mutable. Its [TableWideConstraint](enum.TableWideConstraint.html)s are changed by
/// apllodb ALTER TABLE commands (ADDing / MODIFYing column with T_table_constraint, DROPping column).
///
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
///
/// A vtable is valid during a transaction's lifetime.
#[derive(Hash, Debug)]
pub struct VTable<'tx, Tx: ImmutableSchemaTx> {
    id: VTableId,
    table_wide_constraints: TableWideConstraints,
    version_repo: VersionRepo,

    // Only Table has reference to transaction. Version and Row do not.
    tx: &'tx Tx,
}

impl<Tx: ImmutableSchemaTx> Entity for VTable<'_, Tx> {
    type Id = VTableId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
impl<Tx: ImmutableSchemaTx> PartialEq for VTable<'_, Tx> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<Tx: ImmutableSchemaTx> Eq for VTable<'_, Tx> {}

impl<Tx: ImmutableSchemaTx> PartialOrd for VTable<'_, Tx> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl<Tx: ImmutableSchemaTx> Ord for VTable<'_, Tx> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'tx, Tx: ImmutableSchemaTx<VTbl = Self>> VTable<'tx, Tx> {
    /// Create.
    ///
    /// # Failures
    ///
    /// - Errors from [TableConstraints::new](foo.html).
    pub(crate) fn create(
        tx: &'tx Tx,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<Self> {
        use apllodb_shared_components::traits::Database;

        let constraints = TableWideConstraints::new(table_constraints, column_definitions)?;
        let version = ActiveVersion::create_initial(column_definitions, table_constraints)?;

        let mut version_repo = VersionRepo::default();
        version_repo.add_active_version(version);

        let slf = Self {
            id: VTableId::new(tx.database().name(), table_name),
            table_wide_constraints: constraints,
            version_repo,
            tx,
        };

        let v1 = slf.version_repo.current_version()?;

        // TODO usecaseに仕事させる

        // slf.tx.create_table(&slf)?;
        // slf.tx.create_version(&slf, &v1)?;

        Ok(slf)
    }

    pub(crate) fn alter(&mut self, action: &AlterTableAction) -> ApllodbResult<()> {
        let current_version = self.version_repo.current_version()?;
        let next_version = current_version.create_next(action)?;
        self.version_repo.add_active_version(next_version);

        // TODO auto-upgrade.
        // TODO Inactivate old empty versions.

        self.tx.update_table(&self)?;

        Ok(())
    }

    /// Do the following:
    ///
    /// - List all active versions.
    /// - Resolves each column's ColumnDataType from active versions.
    ///   Note that all active version should have the same data type if the column exists
    ///   (`ALTER TABLE ... MODIFY COLUMN data_type` is not allowed in Immutable Schema).
    /// - Issue SELECT to each active [Version](foobar.html) and get rows.
    ///   If a version does not have a column (which is included in another version), set NULL for its column value.
    /// - Return iterator of rows which produces all the rows in active versions.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [UndefinedColumn](error/enum.ApllodbErrorKind.html#variant.UndefinedColumn) when:
    ///   - At least one `column_names` are not included any active versions.
    pub(crate) fn select(&self, column_names: &[ColumnName]) -> ApllodbResult<Tx::RowIter> {
        let versions = self.versions_for_select();

        // TODO どのバージョンにも含まれないカラム名をチェック

        let _row_iters: Vec<Tx::RowIter> = versions
            .iter()
            .map(|version| {
                let column_data_types: Vec<&ColumnDataType> = column_names
                    .iter()
                    .flat_map(|column_name| version.resolve_column_data_type(column_name))
                    .collect();

                let column_names: Vec<ColumnName> = column_data_types
                    .iter()
                    .map(|cdt| cdt.column_name())
                    .cloned()
                    .collect();

                version.select::<Tx::RowIter>(&column_names)
            })
            .collect::<ApllodbResult<Vec<Tx::RowIter>>>()?;

        // Ok(Tx::RowIter::chain(row_iters))

        todo!()
    }

    /// Ref to TableName.
    ///
    /// Same as `T_create_table_command :: ... :: T_table_name`.
    pub(crate) fn table_name(&self) -> &TableName {
        &self.id.table_name
    }

    /// Ref to TableWideConstraints
    pub(crate) fn table_wide_constraints(&self) -> &TableWideConstraints {
        &self.table_wide_constraints
    }

    fn versions_for_select(&self) -> Vec<ActiveVersion> {
        todo!()
    }

    #[allow(dead_code)]
    fn resolve_column_definition(
        &self,
        _column_name: &ColumnName,
    ) -> ApllodbResult<ColumnDefinition> {
        todo!()
    }
}
