use super::{constraints::VersionConstraints, Version, VersionId, VersionNumber};
use crate::{entity::Entity, vtable::VTableId};
use apllodb_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, ColumnName, TableConstraints,
};
use apllodb_shared_components::{
    data_structure::ColumnDataType,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{Deserialize, Serialize};

/// Active Version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct ActiveVersion(Version);

impl Entity for ActiveVersion {
    type Id = VersionId;

    fn id(&self) -> &Self::Id {
        &self.0.id
    }
}

impl ActiveVersion {
    /// Version number.
    pub fn number(&self) -> &VersionNumber {
        &self.0.id.version_number
    }

    pub fn vtable_id(&self) -> &VTableId {
        &self.0.id.vtable_id
    }

    /// Create v_1.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](variant.InvalidTableDefinition.html)
    ///   - If `column_definitions` is empty.
    pub fn initial(
        vtable_id: &VTableId,
        column_definitions: &[ColumnDefinition],
        table_constraints: &TableConstraints,
    ) -> ApllodbResult<Self> {
        Self::new(
            vtable_id,
            &VersionNumber::initial(),
            column_definitions,
            table_constraints,
        )
    }

    /// Constructor.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](variant.InvalidTableDefinition.html)
    ///   - If `column_definitions` is empty.
    pub fn new(
        vtable_id: &VTableId,
        version_number: &VersionNumber,
        column_definitions: &[ColumnDefinition],
        _table_constraints: &TableConstraints,
    ) -> ApllodbResult<Self> {
        // TODO Check table_constraints when  TableConstraints support constraints per record.
        // TODO Validate integrity between column_definitions & table_dconstraints.

        let column_data_types: Vec<ColumnDataType> =
            column_definitions.iter().map(|d| d.into()).collect();

        Self::validate_at_least_one_column(&column_data_types)?;

        let id = VersionId::new(vtable_id, version_number);

        Ok(Self(Version {
            id,
            column_data_types,
            // TODO: カラム制約とテーブル制約からつくる
            constraints: VersionConstraints::default(),
        }))
    }

    /// Ref to columns and their data types.
    pub fn column_data_types(&self) -> &[ColumnDataType] {
        &self.0.column_data_types
    }

    /// Create v_(current+1) from v_current.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](variant.InvalidTableDefinition.html)
    ///   - If no column would exist after the specified action.
    /// - [UndefinedColumn](variant.UndefinedColumn.html)
    ///   - If column to alter does not exist.
    pub fn create_next(&self, action: &AlterTableAction) -> ApllodbResult<Self> {
        match action {
            AlterTableAction::DropColumn {
                column_name: column_to_drop,
            } => {
                self.validate_col_existence(&column_to_drop)?;

                let next_column_data_types: Vec<ColumnDataType> = self
                    .0
                    .column_data_types
                    .iter()
                    .filter(|c| c.column_name() != column_to_drop)
                    .cloned()
                    .collect();

                // TODO self.constraints のバージョン制約が column_to_drop を含んでいた場合の対処。
                // たぶん、errorを返すんだと思う。

                Self::validate_at_least_one_column(&next_column_data_types)?;

                let id = VersionId::new(self.vtable_id(), &self.number().next());

                Ok(Self(Version {
                    id,
                    column_data_types: next_column_data_types,
                    constraints: VersionConstraints::default(),
                }))
            }
        }
    }

    /// Returns ColumnDataType of `column_name` if this version has it.
    fn resolve_column_data_type(&self, column_name: &ColumnName) -> Option<&ColumnDataType> {
        self.column_data_types()
            .iter()
            .find(|cdt| cdt.column_name() == column_name)
    }

    fn validate_at_least_one_column(column_data_types: &[ColumnDataType]) -> ApllodbResult<()> {
        if column_data_types.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTableDefinition,
                "no column in a table definition.",
                None,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_col_existence(&self, column_name: &ColumnName) -> ApllodbResult<()> {
        self.0
            .column_data_types
            .iter()
            .find(|c| c.column_name() == column_name)
            .map(|_| ())
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("column `{}` does not exist in current version", column_name),
                    None,
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::ActiveVersion;
    use crate::{vtable::VTableId, vtable_id};
    use apllodb_shared_components::error::{ApllodbErrorKind, ApllodbResult};
    use apllodb_shared_components::{
        alter_table_action_drop_column, column_constraints, column_definition, column_name,
        data_structure::{ColumnName, DataTypeKind},
        data_type, table_constraints,
    };

    #[test]
    fn test_initial_success() -> ApllodbResult<()> {
        let column_definitions = vec![column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        )];
        let table_constraints = table_constraints!();

        let v = ActiveVersion::initial(&vtable_id!(), &column_definitions, &table_constraints)?;
        assert_eq!(v.number().to_u64(), 1);

        Ok(())
    }

    #[test]
    fn test_initial_fail_invalid_table_definition() -> ApllodbResult<()> {
        let column_definitions = vec![];
        let table_constraints = table_constraints!();

        match ActiveVersion::initial(&vtable_id!(), &column_definitions, &table_constraints) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::InvalidTableDefinition => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }

    #[test]
    fn test_create_next_drop_column_success() -> ApllodbResult<()> {
        let column_definitions = vec![
            column_definition!(
                "c1",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
            column_definition!(
                "c2",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
        ];
        let table_constraints = table_constraints!();
        let v1 = ActiveVersion::initial(&vtable_id!(), &column_definitions, &table_constraints)?;

        let action = alter_table_action_drop_column!("c1");

        let v2 = v1.create_next(&action)?;

        assert_eq!(v2.number().to_u64(), 2);

        let v2_cols: Vec<ColumnName> = v2
            .column_data_types()
            .iter()
            .map(|cdt| cdt.column_name())
            .cloned()
            .collect();
        assert_eq!(v2_cols, vec![column_name!("c2")]);

        Ok(())
    }

    #[test]
    fn test_create_next_drop_column_fail_undefined_column() -> ApllodbResult<()> {
        let column_definitions = vec![column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        )];
        let table_constraints = table_constraints!();
        let v1 = ActiveVersion::initial(&vtable_id!(), &column_definitions, &table_constraints)?;

        let action = alter_table_action_drop_column!("c404");
        match v1.create_next(&action) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::UndefinedColumn => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }

    #[test]
    fn test_create_next_drop_column_fail_invalid_table_definition() -> ApllodbResult<()> {
        let column_definitions = vec![column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        )];
        let table_constraints = table_constraints!();
        let v1 = ActiveVersion::initial(&vtable_id!(), &column_definitions, &table_constraints)?;

        let action = alter_table_action_drop_column!("c1");
        match v1.create_next(&action) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::InvalidTableDefinition => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }
}
