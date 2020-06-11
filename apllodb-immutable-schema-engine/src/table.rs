mod constraint_kind;
mod constraints;
mod version_repo;

use crate::{
    transaction::{ImmutableSchemaTx, RowIterator},
    version::column::ColumnDataType,
    ActiveVersion,
};
use apllodb_shared_components::{
    data_structure::{AlterTableAction, ColumnDefinition, ColumnName, TableConstraints, TableName},
    error::ApllodbResult,
};
use apllodb_storage_manager_interface::TxCtxLike;
use constraints::TableWideConstraints;
use serde::Serialize;
use std::cmp::Ordering;
use version_repo::VersionRepo;

/// A table, which has set of [Version](struct.Version.html)s.
///
/// A table is mutable. Its [TableWideConstraint](enum.TableWideConstraint.html)s are changed by
/// apllodb ALTER TABLE commands (ADDing / MODIFYing column with T_table_constraint, DROPping column).
///
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
///
/// A table is valid during a transaction's lifetime.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize)] // FIXME make it Deserialize
pub(crate) struct Table<'tx, Tx: ImmutableSchemaTx> {
    name: TableName,
    table_wide_constraints: TableWideConstraints,
    version_repo: VersionRepo,
    tx: &'tx Tx,
}

impl<Tx: ImmutableSchemaTx> PartialOrd for Table<'_, Tx> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<Tx: ImmutableSchemaTx> Ord for Table<'_, Tx> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<'tx, Tx: ImmutableSchemaTx<Tbl = Self>> Table<'tx, Tx> {
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
        let constraints = TableWideConstraints::new(table_constraints, column_definitions)?;
        let version = ActiveVersion::create_initial(column_definitions, table_constraints)?;

        let mut version_repo = VersionRepo::default();
        version_repo.add_active_version(version);

        let slf = Self {
            name: TableName::from(table_name.clone()),
            table_wide_constraints: constraints,
            version_repo,
            tx,
        };

        slf.tx.create_table(&slf)?;

        Ok(slf)
    }

    pub(crate) fn alter(&mut self, action: &AlterTableAction) -> ApllodbResult<()> {
        let current_version = self.version_repo.current_version()?;
        let next_version = current_version.create_next(action)?;
        self.version_repo.add_active_version(next_version);

        // TODO auto-upgrade.
        // TODO Inactivate old empty versions.

        self.tx.alter_table(&self)?;

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
    fn select(&self, column_names: &[ColumnName]) -> ApllodbResult<RowIterator<'tx>> {
        let column_data_types: Vec<ColumnDataType> = column_names
            .iter()
            .map(|column_name| {
                let coldef: ColumnDefinition = self.resolve_column_definition(column_names)?; // どのactive versionも持たないカラムがあれば UndefinedColumn

                ColumnDataType::from(&coldef)
            })
            .collect();

        self.versions_for_select()

        // use crate::transaction::sqlite_tx::from_sqlite_row::FromSqliteRow;

        // let column_data_types: &[ColumnDataType] = todo!(); // TODO カタログから引っ張る

        // let mut stmt = _tx.sqlite_tx.prepare("SELECT * FROM t").unwrap();
        // let sqlite_rows = stmt.query_map_named(&[], |sqlite_row| {
        //     Row::from_sqlite_row(sqlite_row, &column_data_types)?
        // })?;
        // Ok(SqliteRowIterator::from(sqlite_rows))
    }

    /// Ref to TableName.
    ///
    /// Same as `T_create_table_command :: ... :: T_table_name`.
    pub fn name(&self) -> &TableName {
        &self.name
    }

    /// Ref to TableWideConstraints
    pub(crate) fn table_wide_constraints(&self) -> &TableWideConstraints {
        &self.table_wide_constraints
    }

    /// Ref to VersionRepo
    pub(crate) fn version_repo(&self) -> &VersionRepo {
        &self.version_repo
    }
}
