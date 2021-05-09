use super::{constraints::VersionConstraints, version_number::VersionNumber, Version, VersionId};
use crate::{entity::Entity, vtable::id::VTableId};
use apllodb_shared_components::{ApllodbError, ApllodbResult, SqlState, SqlValue};
use apllodb_storage_engine_interface::{AlterTableAction, ColumnDataType, ColumnName};
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
    pub fn initial(
        vtable_id: &VTableId,
        non_pk_column_data_types: &[ColumnDataType],
    ) -> ApllodbResult<Self> {
        Self::new(
            vtable_id,
            &VersionNumber::initial(),
            non_pk_column_data_types,
            // TODO constraints from TableConstraints
            VersionConstraints::default(),
        )
    }

    /// Constructor.
    pub fn new(
        vtable_id: &VTableId,
        version_number: &VersionNumber,
        non_pk_column_data_types: &[ColumnDataType],
        version_constraints: VersionConstraints,
    ) -> ApllodbResult<Self> {
        let id = VersionId::new(vtable_id, version_number);

        Ok(Self(Version {
            id,
            column_data_types: non_pk_column_data_types.to_vec(),
            constraints: version_constraints,
        }))
    }

    /// Ref to columns and their data types.
    pub fn column_data_types(&self) -> &[ColumnDataType] {
        &self.0.column_data_types
    }

    /// Ref to version constraints.
    pub fn version_constraints(&self) -> &VersionConstraints {
        &self.0.constraints
    }

    /// Create v_(current+1) from v_current.
    ///
    /// # Failures
    ///
    /// - [DdlError](variant.DdlError.html)
    ///   - If no column would exist after the specified action.
    /// - [NameErrorNotFound](variant.NameErrorNotFound.html)
    ///   - If column to alter does not exist.
    pub fn create_next(&self, action: &AlterTableAction) -> ApllodbResult<Self> {
        match action {
            AlterTableAction::AddColumn {
                column_definition: cd_to_add,
            } => {
                self.validate_col_not_exists(cd_to_add.column_data_type().column_name())?;

                let mut next_column_data_types: Vec<ColumnDataType> =
                    self.0.column_data_types.clone();
                next_column_data_types.push(cd_to_add.column_data_type().clone());

                // TODO treat cd_to_add.column_constraint

                let id = VersionId::new(self.vtable_id(), &self.number().next());

                Ok(Self(Version {
                    id,
                    column_data_types: next_column_data_types,
                    constraints: VersionConstraints::default(),
                }))
            }
            AlterTableAction::DropColumn {
                column_name: column_to_drop,
            } => {
                self.validate_col_exists(&column_to_drop)?;

                let next_column_data_types: Vec<ColumnDataType> = self
                    .0
                    .column_data_types
                    .iter()
                    .filter(|cdt| cdt.column_name().as_str() != column_to_drop.as_str())
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
    /// - [NotNullViolation](apllodb_shared_components::SqlState::NotNullViolation) when:
    ///   - Not inserting into a NOT NULL column.
    ///   - Inserting NULL to column with NOT NULL constraint.
    /// - [NameErrorNotFound](apllodb-shared-components::SqlState::NameErrorNotFound) when:
    ///   - `column_values` includes any column not defined in this version.
    /// - [CheckViolation](apllodb_shared_components::SqlState::CheckViolation) when:
    ///   - Column value does not satisfy CHECK constraint.
    pub(in crate::version) fn check_version_constraint(
        &self,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()> {
        let version_column_data_types = self.column_data_types();

        // Check if all NOT NULL columns are included in `column_values`.
        let version_not_null_columns = version_column_data_types
            .iter()
            .filter(|cdt| !cdt.nullable());
        for not_null_column_name in version_not_null_columns.map(|cdt| cdt.column_name()) {
            if !column_values.contains_key(not_null_column_name) {
                return Err(ApllodbError::integrity_constraint_not_null_violation(
                    format!(
                        "column `{:?}` (NOT NULL) must be included in INSERT command",
                        not_null_column_name
                    ),
                ));
            }
        }

        let version_column_names: Vec<&ColumnName> = version_column_data_types
            .iter()
            .map(|cdt| cdt.column_name())
            .collect();

        // Check if all columns in `column_values` are included in version's definition.
        for cn in column_values.keys() {
            if !version_column_names.contains(&cn) {
                return Err(ApllodbError::name_error_not_found(format!(
                    "inserted column `{:?}` is not defined in version `{:?}`",
                    cn, self
                )));
            }
        }

        // Check column value to insert.
        // for (_column_name, _expr) in column_values {
        // TODO implement NullViolation error detection after Expression can hold NULL.
        // TODO implement CheckViolation error detection
        // }

        Ok(())
    }

    fn validate_col_not_exists(&self, column_name: &ColumnName) -> ApllodbResult<()> {
        if self
            .0
            .column_data_types
            .iter()
            .any(|cdt| cdt.column_name() == column_name)
        {
            Err(ApllodbError::name_error_duplicate(format!(
                "column `{:?}` already exists in current version",
                column_name
            )))
        } else {
            Ok(())
        }
    }

    fn validate_col_exists(&self, column_name: &ColumnName) -> ApllodbResult<()> {
        self.0
            .column_data_types
            .iter()
            .find(|cdt| cdt.column_name() == column_name)
            .map(|_| ())
            .ok_or_else(|| {
                ApllodbError::new(
                    SqlState::NameErrorNotFound,
                    format!(
                        "column `{:?}` does not exist in current version",
                        column_name
                    ),
                    None,
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::ActiveVersion;
    use crate::vtable::id::VTableId;
    use apllodb_shared_components::{ApllodbResult, SqlState, SqlType};
    use apllodb_storage_engine_interface::{AlterTableAction, ColumnDataType, ColumnName};

    #[test]
    fn test_initial_success() -> ApllodbResult<()> {
        let c1_cdt = ColumnDataType::factory("c1", SqlType::integer(), false);

        let v = ActiveVersion::initial(&VTableId::new_for_test(), &[c1_cdt])?;
        assert_eq!(v.number().to_u64(), 1);

        Ok(())
    }

    #[test]
    fn test_create_next_drop_column_success() -> ApllodbResult<()> {
        let c1_cdt = ColumnDataType::factory("c1", SqlType::integer(), false);
        let c2_cdt = ColumnDataType::factory("c2", SqlType::integer(), false);

        let column_data_types = vec![c1_cdt.clone(), c2_cdt.clone()];

        let v1 = ActiveVersion::initial(&VTableId::new_for_test(), &column_data_types)?;

        let action = AlterTableAction::DropColumn {
            column_name: c1_cdt.column_name().clone(),
        };

        let v2 = v1.create_next(&action)?;

        assert_eq!(v2.number().to_u64(), 2);

        let v2_cols: Vec<&ColumnName> = v2
            .column_data_types()
            .iter()
            .map(|cdt| cdt.column_name())
            .collect();
        assert_eq!(v2_cols, vec![c2_cdt.column_name()]);

        Ok(())
    }

    #[test]
    fn test_create_next_drop_column_fail_undefined_column() -> ApllodbResult<()> {
        let c1_cdt = ColumnDataType::factory("c1", SqlType::integer(), false);
        let v1 = ActiveVersion::initial(&VTableId::new_for_test(), &[c1_cdt])?;

        let action = AlterTableAction::DropColumn {
            column_name: ColumnName::factory("c404"),
        };
        match v1.create_next(&action) {
            Err(e) => match e.kind() {
                SqlState::NameErrorNotFound => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }
}
