mod action;
mod column;
mod constraint;

use action::NextVersionAction;
use apllo_shared_components::data_structure::{ColumnDefinition, ColumnName, TableConstraints};
use apllo_shared_components::error::{AplloError, AplloErrorKind, AplloResult};
use column::ColumnDataType;
use constraint::VersionConstraint;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

/// Version.
///
/// A version belongs to a [VersionSet](struct.VersionSet.html).
/// A version directly has subset of records in the VersionSet.
///
/// - The version `v_1` is created by APLLO CREATE TABLE command.
/// - Version `v_(current+1)` is created by APLLO ALTER TABLE command.
/// - Some of `v_1` ~ `v_current` are deactivated by APLLO ALTER TABLE command
///   if all the records in `v_i` can be migrated to `v_(current+1)` (auto upgrade).
/// - All of `v_1` ~ `v_current` are deactivated by APLLO DROP TABLE command.
///
/// Each version purely immutable.
///
/// See: https://github.com/darwin-education/apllo/wiki/Immutable-Schema-102:-Immutable-Schema-%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E5%AE%9A%E7%BE%A9%E3%83%BB%E5%AE%9A%E7%90%86
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
    /// Create v_1.
    pub(crate) fn create_initial(
        column_definitions: &[ColumnDefinition],
        _table_constraints: &TableConstraints,
    ) -> Self {
        Self {
            number: 1,
            column_data_types: column_definitions.iter().map(|d| d.into()).collect(),
            // TODO: カラム制約とテーブル制約からつくる
            constraints: vec![],
        }
    }

    /// Create v_(current+1) from v_current.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](variant.UndefinedColumn.html)
    ///   - If column to alter does not exist.
    pub(crate) fn create_next(&self, action: NextVersionAction) -> AplloResult<Self> {
        let number = self.number + 1;

        match action {
            NextVersionAction::DropColumn {
                column: column_to_drop,
            } => {
                self.validate_col_existence(&column_to_drop)?;

                let next_column_data_types: Vec<ColumnDataType> = self
                    .column_data_types
                    .iter()
                    .filter(|c| c.column != column_to_drop)
                    .cloned()
                    .collect();

                // TODO self.constraints のバージョン制約が column_to_drop を含んでいた場合の対処。
                // たぶん、errorを返すんだと思う。

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

    fn validate_col_existence(&self, column_name: &ColumnName) -> AplloResult<()> {
        self.column_data_types
            .iter()
            .find(|c| &c.column == column_name)
            .map(|_| ())
            .ok_or_else(|| {
                AplloError::new(
                    AplloErrorKind::UndefinedColumn,
                    format!("column `{}` does not exist in current version", column_name),
                    None,
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn test_create_initial_success() {
        // let column_definitions = vec![ColumnDefinition()];
        // Version::create_initial(column_definitions, &[])

        // TODO そろそろfixtureつくる。ColumnDefinitionつくるのだけで一苦労なので...
    }
}