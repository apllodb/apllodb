use super::constraint_kind::VersionSetConstraintKind;
use crate::versions::validation_helper::collection::find_dup;
use apllo_shared_components::{
    data_structure::{ColumnDefinition, ColumnName, TableConstraints},
    error::{AplloError, AplloErrorKind, AplloResult},
};
use serde::{Deserialize, Serialize};

/// VersionSet constraints.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(super) struct VersionSetConstraints {
    kinds: Vec<VersionSetConstraintKind>,
}
impl VersionSetConstraints {
    /// Constructor that extracts VersionSet constraints (set of record must obey)
    /// from TableConstraints and ColumnConstraints in each ColumnDefinition.
    ///
    /// # Failures
    /// - [InvalidTableDefinition](error/enum.AplloErrorKind.html#variant.InvalidTableDefinition) when:
    ///   - [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) or
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) in `table_constraints` and `column_definitions`
    ///     are applied to the same single column.
    ///   - Both `table_constraints` and `column_definitions` include [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey).
    pub(super) fn new(
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> AplloResult<Self> {
        use std::convert::TryFrom;

        let from_table_constraints = table_constraints
            .kinds()
            .iter()
            .map(VersionSetConstraintKind::from);

        let from_column_definitions = column_definitions
            .iter()
            .flat_map(VersionSetConstraintKind::try_from);

        let kinds: Vec<VersionSetConstraintKind> = from_table_constraints
            .chain(from_column_definitions)
            .collect();

        Self::validate_pk_duplication(&kinds)?;
        Self::validate_pk_or_unique_target_duplication(&kinds)?;

        Ok(Self { kinds })
    }

    fn validate_pk_duplication(kinds: &[VersionSetConstraintKind]) -> AplloResult<()> {
        if kinds
            .iter()
            .filter(|kind| match kind {
                VersionSetConstraintKind::PrimaryKey { .. } => true,
                _ => false,
            })
            .count()
            > 1
        {
            Err(AplloError::new(
                AplloErrorKind::InvalidTableDefinition,
                "more than 1 PRIMARY KEY are specified",
                None,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_pk_or_unique_target_duplication(
        kinds: &[VersionSetConstraintKind],
    ) -> AplloResult<()> {
        let single_columns: Vec<&ColumnName> = kinds
            .iter()
            .flat_map(|k| match k {
                VersionSetConstraintKind::PrimaryKey { column_names } => {
                    if column_names.len() == 1 {
                        column_names.first()
                    } else {
                        None
                    }
                }
                VersionSetConstraintKind::Unique { column_names } => {
                    if column_names.len() == 1 {
                        column_names.first()
                    } else {
                        None
                    }
                }
            })
            .collect();
        if let Some(column) = find_dup(single_columns.iter()) {
            Err(AplloError::new(
                AplloErrorKind::InvalidTableDefinition,
                format!(
                    "more than 1 PRIMARY KEY / UNIQUE are applied to the same column: `{:?}`",
                    column
                ),
                None,
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VersionSetConstraints;
    use crate::{column_constraints, column_definition, t_pk, t_unique, table_constraints};
    use apllo_shared_components::{
        data_structure::{ColumnDefinition, TableConstraints},
        error::AplloErrorKind,
    };

    #[test]
    fn test_success() {
        let testset: Vec<(TableConstraints, Vec<ColumnDefinition>)> = vec![
            (
                table_constraints!(),
                vec![column_definition!("c1", column_constraints!())],
            ),
            (
                table_constraints!(t_pk!("c1"), t_unique!("c2")),
                vec![
                    column_definition!("c1", column_constraints!()),
                    column_definition!("c2", column_constraints!()),
                ],
            ),
            (
                table_constraints!(t_pk!("c1")),
                vec![
                    column_definition!("c1", column_constraints!()),
                    column_definition!("c2", column_constraints!(ColumnConstraintKind::Unique)),
                ],
            ),
            (
                table_constraints!(t_pk!("c2", "c1")),
                vec![
                    column_definition!("c1", column_constraints!()),
                    column_definition!("c2", column_constraints!(ColumnConstraintKind::Unique)),
                ],
            ),
        ];

        for (table_constraints, column_definitions) in testset {
            match VersionSetConstraints::new(&table_constraints, &column_definitions) {
                Ok(_) => {}
                Err(e) => panic!("unexpected error kind: {}", e),
            }
        }
    }

    #[test]
    fn test_failure_invalid_table_definition() {
        let testset: Vec<(TableConstraints, Vec<ColumnDefinition>)> = vec![
            (
                table_constraints!(t_pk!("c1")),
                vec![column_definition!(
                    "c1",
                    column_constraints!(ColumnConstraintKind::Unique)
                )],
            ),
            (
                table_constraints!(t_unique!("c1")),
                vec![column_definition!(
                    "c1",
                    column_constraints!(ColumnConstraintKind::Unique)
                )],
            ),
            (
                table_constraints!(t_pk!("c1")),
                vec![column_definition!(
                    "c2",
                    column_constraints!(ColumnConstraintKind::PrimaryKey)
                )],
            ),
        ];

        for (table_constraints, column_definitions) in testset {
            match VersionSetConstraints::new(&table_constraints, &column_definitions) {
                Err(e) => match e.kind() {
                    AplloErrorKind::InvalidTableDefinition => {
                        println!("{}", e);
                    }
                    _ => panic!("unexpected error kind: {}", e),
                },
                Ok(tc) => panic!("Expected to be validation error: {:?}", tc),
            }
        }
    }
}
