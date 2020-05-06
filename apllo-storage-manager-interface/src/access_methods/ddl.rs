use crate::{Version, VersionSet};
use apllo_shared_components::{ColumnDefinition, TableConstraint, TableName};

/// Access methods for DDL.
///
/// A storage engine must implement interface functions.
pub trait AccessMethodsDdl {
    // TODO async とかつけような

    /// CREATE TABLE command.
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
    fn create_table(
        table_name: &TableName,
        table_constraints: &[TableConstraint],
        column_definitions: &[ColumnDefinition],
    ) {
        let version_set = VersionSet::new(table_name, table_constraints, column_definitions);
        let version = Version::create_initial(column_definitions, table_constraints);

        Self::materialize_version_set(version_set);
        Self::materialize_version(version);
    }

    /// ALTER TABLE command.
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
    fn alter_table();

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
    fn drop_table();

    fn materialize_version_set(version_set: VersionSet);

    fn materialize_version(version: Version);
}
