use super::{constraints::VersionConstraints, Version, VersionId, VersionNumber};
use crate::{entity::Entity, vtable::VTableId,  ApparentPrimaryKeyColumnNames};
use apllodb_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, ColumnName,
};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, Expression},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        apk_column_names: &ApparentPrimaryKeyColumnNames,
        column_definitions: &[ColumnDefinition],
        // TODO constraints from TableConstraints
    ) -> ApllodbResult<Self> {
        Self::new(
            vtable_id,
            &VersionNumber::initial(),
            apk_column_names,
            column_definitions,
        )
    }

    /// Constructor.
    ///
    /// Columns from column_definitions are filtered-out if they are included in apk_column_names.
    pub fn new(
        vtable_id: &VTableId,
        version_number: &VersionNumber,
        apk_column_names: &ApparentPrimaryKeyColumnNames,
        column_definitions: &[ColumnDefinition],
        // TODO constraints from TableConstraints
    ) -> ApllodbResult<Self> {
        let apk_column_names = apk_column_names.column_names();

        let column_data_types: Vec<ColumnDataType> = column_definitions
            .iter()
            .map(ColumnDataType::from)
            // Filter-out primary key columns.
            .filter(|cdt| !apk_column_names.contains(cdt.column_name()))
            .collect();

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

                let id = VersionId::new(self.vtable_id(), &self.number().next());

                Ok(Self(Version {
                    id,
                    column_data_types: next_column_data_types,
                    constraints: VersionConstraints::default(),
                }))
            }
        }
    }

    /// Returns the biggest version that can accept `column_values`.
    ///
    /// # Failures
    ///
    /// - [NotNullViolation](error/enum.ApllodbErrorKind.html#variant.NotNullViolation) when:
    ///   - Not inserting into a NOT NULL column.
    ///   - Inserting NULL to column with NOT NULL constraint.
    /// - [CheckViolation](error/enum.ApllodbErrorKind.html#variant.CheckViolation) when:
    ///   - Column value does not satisfy CHECK constraint.
    pub(in crate::version) fn check_version_constraint(
        &self,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let column_data_types = self.column_data_types();

        // Check if any column not to be inserted is NOT NULL.
        let not_null_columns = column_data_types
            .iter()
            .filter(|cdt| !cdt.data_type().nullable());
        for not_null_column_name in not_null_columns.map(|cdt| cdt.column_name()) {
            if !column_values.contains_key(not_null_column_name) {
                return Err(ApllodbError::new(
                    ApllodbErrorKind::NotNullViolation,
                    format!(
                        "column `{}` (NOT NULL) must be included in INSERT command",
                        not_null_column_name,
                    ),
                    None,
                ));
            }
        }

        // Check column value to insert.
        for (_column_name, _expr) in column_values {

            // TODO implement NullViolation error detection after Expression can hold NULL.

            // TODO implement CheckViolation error detection
        }

        Ok(())
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
    use crate::{test_support::setup, vtable_id, apk_column_names};
    use apllodb_shared_components::error::{ApllodbErrorKind, ApllodbResult};
    use apllodb_shared_components::{
        alter_table_action_drop_column, column_constraints, column_definition, column_name,
        data_structure::{ColumnName, DataTypeKind},
        data_type, 
    };

    #[test]
    fn test_initial_success() -> ApllodbResult<()> {
        setup();

        let apk_column_names = apk_column_names!("c1");
        let column_definitions = vec![column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        )];

        let v = ActiveVersion::initial(&vtable_id!(), &apk_column_names, &column_definitions)?;
        assert_eq!(v.number().to_u64(), 1);

        Ok(())
    }

    #[test]
    fn test_create_next_drop_column_success() -> ApllodbResult<()> {
        setup();

        let apk_column_names = apk_column_names!("id");
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

        let v1 = ActiveVersion::initial(&vtable_id!(), &apk_column_names, &column_definitions)?;

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
        setup();

        let apk_column_names = apk_column_names!("id");
        let column_definitions = vec![column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        )];
        let v1 = ActiveVersion::initial(&vtable_id!(), &apk_column_names, &column_definitions)?;

        let action = alter_table_action_drop_column!("c404");
        match v1.create_next(&action) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::UndefinedColumn => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }
}
