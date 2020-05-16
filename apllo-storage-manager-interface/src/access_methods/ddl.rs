use crate::{Version, VersionSet};
use apllo_shared_components::data_structure::{ColumnDefinition, TableConstraints, TableName};
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
    /// This function does not execute auto-upgrade.
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
    fn alter_table() -> AplloResult<()>;

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
}
