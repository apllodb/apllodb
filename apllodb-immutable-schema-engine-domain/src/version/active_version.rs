use super::{constraints::VersionConstraints, version_number::VersionNumber, Version, VersionId};
use crate::{entity::Entity, vtable::id::VTableId};
use apllodb_shared_components::data_structure::{AlterTableAction, ColumnDataType, ColumnName};
use apllodb_shared_components::{
    data_structure::Expression,
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
    pub fn initial(
        vtable_id: &VTableId,
        non_pk_column_data_types: &[ColumnDataType],
        // TODO constraints from TableConstraints
    ) -> ApllodbResult<Self> {
        Self::new(
            vtable_id,
            &VersionNumber::initial(),
            non_pk_column_data_types,
        )
    }

    /// Constructor.
    pub fn new(
        vtable_id: &VTableId,
        version_number: &VersionNumber,
        non_pk_column_data_types: &[ColumnDataType],
        // TODO constraints from TableConstraints
    ) -> ApllodbResult<Self> {
        let id = VersionId::new(vtable_id, version_number);

        Ok(Self(Version {
            id,
            column_data_types: non_pk_column_data_types.to_vec(),
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
                    .filter(|c| c.column_ref().as_column_name().as_str() != column_to_drop.as_str())
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
    /// - [InvalidColumnReference](apllodb-shared-components::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `column_values` includes any column not defined in this version.
    /// - [CheckViolation](error/enum.ApllodbErrorKind.html#variant.CheckViolation) when:
    ///   - Column value does not satisfy CHECK constraint.
    pub(in crate::version) fn check_version_constraint(
        &self,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let version_column_data_types = self.column_data_types();

        // Check if all NOT NULL columns are included in `column_values`.
        let version_not_null_columns = version_column_data_types
            .iter()
            .filter(|cdt| !cdt.data_type().nullable());
        for not_null_column_name in version_not_null_columns.map(|cdt| cdt.column_ref()) {
            if !column_values.contains_key(not_null_column_name.as_column_name()) {
                return Err(ApllodbError::new(
                    ApllodbErrorKind::NotNullViolation,
                    format!(
                        "column `{}` (NOT NULL) must be included in INSERT command",
                        not_null_column_name
                    ),
                    None,
                ));
            }
        }

        let version_column_names: Vec<&ColumnName> = version_column_data_types
            .iter()
            .map(|cdt| cdt.column_ref().as_column_name())
            .collect();

        // Check if all columns in `column_values` are included in version's definition.
        for (cn, _) in column_values {
            if !version_column_names.contains(&cn) {
                return Err(ApllodbError::new(
                    ApllodbErrorKind::InvalidColumnReference,
                    format!(
                        "inserted column `{}` is not defined in version `{:?}`",
                        cn, self
                    ),
                    None,
                ));
            }
        }

        // Check column value to insert.
        // for (_column_name, _expr) in column_values {
        // TODO implement NullViolation error detection after Expression can hold NULL.
        // TODO implement CheckViolation error detection
        // }

        Ok(())
    }

    fn validate_col_existence(&self, column_name: &ColumnName) -> ApllodbResult<()> {
        self.0
            .column_data_types
            .iter()
            .find(|c| c.column_ref().as_column_name() == column_name)
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
    use crate::{test_support::setup, vtable::id::VTableId};
    use apllodb_shared_components::data_structure::{
        AlterTableAction, ColumnDataType, ColumnName, ColumnReference, DataType, DataTypeKind,
        TableName,
    };
    use apllodb_shared_components::error::{ApllodbErrorKind, ApllodbResult};

    #[test]
    fn test_initial_success() -> ApllodbResult<()> {
        setup();

        let c1_cdt = ColumnDataType::new(
            ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?),
            DataType::new(DataTypeKind::Integer, false),
        );

        let v = ActiveVersion::initial(&VTableId::new_for_test(), &[c1_cdt])?;
        assert_eq!(v.number().to_u64(), 1);

        Ok(())
    }

    #[test]
    fn test_create_next_drop_column_success() -> ApllodbResult<()> {
        setup();

        let c1_cdt = ColumnDataType::new(
            ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?),
            DataType::new(DataTypeKind::Integer, false),
        );
        let c2_cdt = ColumnDataType::new(
            ColumnReference::new(TableName::new("t")?, ColumnName::new("c2")?),
            DataType::new(DataTypeKind::Integer, false),
        );

        let column_data_types = vec![c1_cdt.clone(), c2_cdt.clone()];

        let v1 = ActiveVersion::initial(&VTableId::new_for_test(), &column_data_types)?;

        let action = AlterTableAction::DropColumn {
            column_name: c1_cdt.column_ref().as_column_name().clone(),
        };

        let v2 = v1.create_next(&action)?;

        assert_eq!(v2.number().to_u64(), 2);

        let v2_cols: Vec<&ColumnReference> = v2
            .column_data_types()
            .iter()
            .map(|cdt| cdt.column_ref())
            .collect();
        assert_eq!(v2_cols, vec![c2_cdt.column_ref()]);

        Ok(())
    }

    #[test]
    fn test_create_next_drop_column_fail_undefined_column() -> ApllodbResult<()> {
        setup();

        let c1_cdt = ColumnDataType::new(
            ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?),
            DataType::new(DataTypeKind::Integer, false),
        );
        let v1 = ActiveVersion::initial(&VTableId::new_for_test(), &[c1_cdt])?;

        let action = AlterTableAction::DropColumn {
            column_name: ColumnName::new("c404")?,
        };
        match v1.create_next(&action) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::UndefinedColumn => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }
}
