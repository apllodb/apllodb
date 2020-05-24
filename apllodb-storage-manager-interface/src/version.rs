mod action;
mod column;
mod constraint;

use action::NextVersionAction;
use apllodb_shared_components::data_structure::{ColumnDefinition, ColumnName, TableConstraints};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use column::ColumnDataType;
use constraint::VersionConstraint;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

/// Version.
///
/// A version belongs to a [Table](struct.Table.html).
/// A version directly has subset of records in the Table.
///
/// - The version `v_1` is created by apllodb CREATE TABLE command.
/// - Version `v_(current+1)` is created by apllodb ALTER TABLE command.
/// - Some of `v_1` ~ `v_current` are inactivated by apllodb ALTER TABLE command
///   if all the records in `v_i` can be migrated to `v_(current+1)` (auto upgrade).
/// - All of `v_1` ~ `v_current` are inactivated by apllodb DROP TABLE command.
///
/// Each version is purely immutable.
///
/// See: https://github.com/darwin-education/apllodb/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Version {
    number: u64,
    column_data_types: Vec<ColumnDataType>,
    constraints: Vec<VersionConstraint>,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.number)
    }
}

impl Version {
    /// Version number.
    pub fn number(&self) -> u64 {
        self.number
    }

    /// Ref to columns and their data types.
    pub fn column_data_types(&self) -> &[ColumnDataType] {
        &self.column_data_types
    }
}

impl Version {
    /// Create v_1.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](variant.InvalidTableDefinition.html)
    ///   - If `column_definitions` is empty.
    pub(crate) fn create_initial(
        column_definitions: &[ColumnDefinition],
        _table_constraints: &TableConstraints,
    ) -> ApllodbResult<Self> {
        // TODO Check table_constraints when  TableConstraints support constraints per record.
        // TODO Validate integrity between column_definitions & table_dconstraints.

        let column_data_types: Vec<ColumnDataType> =
            column_definitions.iter().map(|d| d.into()).collect();

        Self::validate_at_least_one_column(&column_data_types)?;

        Ok(Self {
            number: 1,
            column_data_types,
            // TODO: カラム制約とテーブル制約からつくる
            constraints: vec![],
        })
    }

    /// Create v_(current+1) from v_current.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](variant.InvalidTableDefinition.html)
    ///   - If no column would exist after the specified action.
    /// - [UndefinedColumn](variant.UndefinedColumn.html)
    ///   - If column to alter does not exist.
    pub(crate) fn create_next(&self, action: NextVersionAction) -> ApllodbResult<Self> {
        let number = self.number + 1;

        match action {
            NextVersionAction::DropColumn {
                column: column_to_drop,
            } => {
                self.validate_col_existence(&column_to_drop)?;

                let next_column_data_types: Vec<ColumnDataType> = self
                    .column_data_types
                    .iter()
                    .filter(|c| c.column_name() != &column_to_drop)
                    .cloned()
                    .collect();

                // TODO self.constraints のバージョン制約が column_to_drop を含んでいた場合の対処。
                // たぶん、errorを返すんだと思う。

                Self::validate_at_least_one_column(&next_column_data_types)?;

                Ok(Self {
                    number,
                    column_data_types: next_column_data_types,
                    constraints: vec![],
                })
            }

            NextVersionAction::AddColumn {
                column_data_type: _,
                column_constraints: _,
            } => todo!(),
        }
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
        self.column_data_types
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
    use super::Version;
    use crate::{
        column_constraints, column_definition, column_name, next_version_action_drop_column,
        table_constraints,
    };
    use apllodb_shared_components::{
        data_structure::ColumnName,
        error::{ApllodbErrorKind, ApllodbResult},
    };

    #[test]
    fn test_create_initial_success() -> ApllodbResult<()> {
        let column_definitions = vec![column_definition!("c1", column_constraints!())];
        let table_constraints = table_constraints!();

        let v = Version::create_initial(&column_definitions, &table_constraints)?;
        assert_eq!(v.number(), 1);

        Ok(())
    }

    #[test]
    fn test_create_initial_fail_invalid_table_definition() -> ApllodbResult<()> {
        let column_definitions = vec![];
        let table_constraints = table_constraints!();

        match Version::create_initial(&column_definitions, &table_constraints) {
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
            column_definition!("c1", column_constraints!()),
            column_definition!("c2", column_constraints!()),
        ];
        let table_constraints = table_constraints!();
        let v1 = Version::create_initial(&column_definitions, &table_constraints)?;

        let action = next_version_action_drop_column!("c1");

        let v2 = v1.create_next(action)?;

        assert_eq!(v2.number(), 2);

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
        let column_definitions = vec![column_definition!("c1", column_constraints!())];
        let table_constraints = table_constraints!();
        let v1 = Version::create_initial(&column_definitions, &table_constraints)?;

        let action = next_version_action_drop_column!("c404");
        match v1.create_next(action) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::UndefinedColumn => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }

    #[test]
    fn test_create_next_drop_column_fail_invalid_table_definition() -> ApllodbResult<()> {
        let column_definitions = vec![column_definition!("c1", column_constraints!())];
        let table_constraints = table_constraints!();
        let v1 = Version::create_initial(&column_definitions, &table_constraints)?;

        let action = next_version_action_drop_column!("c1");
        match v1.create_next(action) {
            Err(e) => match e.kind() {
                ApllodbErrorKind::InvalidTableDefinition => Ok(()),
                _ => panic!("unexpected error kind: {}", e),
            },
            Ok(_) => panic!("should be error"),
        }
    }
}
