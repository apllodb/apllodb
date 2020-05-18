use crate::{Version, VersionSet, VersionSetName};
use apllo_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, TableConstraints, TableName,
};
use apllo_shared_components::error::AplloResult;

/// Access methods for DDL.
///
/// A storage engine must implement interface functions.
pub trait AccessMethodsDdl {
    // TODO async とかつけような

    /// CREATE TABLE command.
    ///
    /// # Failures
    ///
    /// - Errors from [VersionSet::new](foobar.html).
    /// - Errors from [Version::create_initial](foobar.html).
    /// - Errors from [materialize_version_set](method.materialize_version_set.html).
    /// - Errors from [materialize_version](method.materialize_version.html).
    fn create_table(
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> AplloResult<()> {
        let version_set = VersionSet::new(table_name, table_constraints, column_definitions)?;
        let version = Version::create_initial(column_definitions, table_constraints)?;

        Self::materialize_version_set(version_set)?;
        Self::materialize_version(version)?;

        Ok(())
    }

    /// ALTER TABLE command.
    ///
    /// This function executes the following steps:
    ///
    /// 1. Dematerialize `v_current`.
    /// 1. Create `v_(current+1)`.
    /// 1. Auto-upgrade.
    /// 1. Deactivate `v_i` `(i <= current)` if all of `v_i`'s records are DELETEd.
    ///
    /// # Failures
    ///
    fn alter_table(table_name: &TableName, action: &AlterTableAction) -> AplloResult<()> {
        // TODO transaction (lock)

        let version_set =
            Self::dematerialize_version_set(&VersionSetName::from(table_name.clone()))?;
        let current_version_num = version_set.current_version_number();
        let current_version = Self::dematerialize_active_version(current_version_num)?;

        let alter_version_set_action = AlterVersionSetAction::from(action);
        let next_version_action = NextVersionAction::from(action);

        version_set.alter(alter_version_set_action)?;
        let next_version = current_version.create_next(next_version_action)?;

        // TODO auto-upgrade.
        // TODO Deactivate old empty versions.

        Self::materialize_version_set(version_set)?;
        Self::materialize_version(next_version)?;

        Ok(())
    }

    /// DROP TABLE command.
    ///
    /// # Panics
    ///
    /// # Failures
    ///
    /// # Safety
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    fn drop_table() -> AplloResult<()>;

    fn materialize_version_set(version_set: VersionSet) -> AplloResult<()>;

    fn materialize_version(version: Version) -> AplloResult<()>;

    fn dematerialize_version_set(name: &VersionSetName) -> AplloResult<VersionSet>;

    fn dematerialize_active_version(version_number: u64) -> AplloResult<Version>;
}
