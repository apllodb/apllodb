use super::constraint_kind::TableWideConstraintKind;
use crate::helper::collection_validation::find_dup;
use apllodb_shared_components::{
    data_structure::{ColumnDefinition, ColumnName, TableConstraints},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{Deserialize, Serialize};

/// Table-wide constraints applied to record set.
///
/// Note that "table constraint" used mainly in syntax (`T_tableConstraint`) and "table-wide constraint" are different.
/// The former is "a constraint applied to the table itself or multiple columns". (e.g. FOREIGN KEY is a table constraint)
/// The latter is "a constraint applied to set of records". (e.g. FOREIGN KEY is NOT a table-wide constraint but a version constraint)
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub(super) struct TableWideConstraints {
    kinds: Vec<TableWideConstraintKind>,
}
impl TableWideConstraints {
    /// Constructor that extracts Table constraints (set of record must obey)
    /// from TableConstraints and ColumnConstraints in each ColumnDefinition.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](error/enum.ApllodbErrorKind.html#variant.InvalidTableDefinition) when:
    ///   - [PrimaryKey](enum.TableWideConstraintKind.html#variant.PrimaryKey) or
    ///     [Unique](enum.TableWideConstraintKind.html#variant.Unique) in `table_constraints` and `column_definitions`
    ///     are applied to the same single column.
    ///   - Both `table_constraints` and `column_definitions` include [PrimaryKey](enum.TableWideConstraintKind.html#variant.PrimaryKey).
    pub(super) fn new(
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<Self> {
        use std::convert::TryFrom;

        let from_table_constraints = table_constraints
            .kinds()
            .iter()
            .map(TableWideConstraintKind::from);

        let from_column_definitions = column_definitions
            .iter()
            .flat_map(TableWideConstraintKind::try_from);

        let kinds: Vec<TableWideConstraintKind> = from_table_constraints
            .chain(from_column_definitions)
            .collect();

        Self::validate_pk_duplication(&kinds)?;
        Self::validate_pk_or_unique_target_duplication(&kinds)?;

        Ok(Self { kinds })
    }

    fn validate_pk_duplication(kinds: &[TableWideConstraintKind]) -> ApllodbResult<()> {
        if kinds
            .iter()
            .filter(|kind| match kind {
                TableWideConstraintKind::PrimaryKey { .. } => true,
                _ => false,
            })
            .count()
            > 1
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTableDefinition,
                "more than 1 PRIMARY KEY are specified",
                None,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_pk_or_unique_target_duplication(
        kinds: &[TableWideConstraintKind],
    ) -> ApllodbResult<()> {
        let single_columns: Vec<&ColumnName> = kinds
            .iter()
            .flat_map(|k| match k {
                TableWideConstraintKind::PrimaryKey { column_names } => {
                    if column_names.len() == 1 {
                        column_names.first()
                    } else {
                        None
                    }
                }
                TableWideConstraintKind::Unique { column_names } => {
                    if column_names.len() == 1 {
                        column_names.first()
                    } else {
                        None
                    }
                }
            })
            .collect();
        if let Some(column) = find_dup(single_columns.iter()) {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTableDefinition,
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
    use super::TableWideConstraints;
    use crate::{column_constraints, column_definition, t_pk, t_unique, table_constraints};
    use apllodb_shared_components::{
        data_structure::{ColumnDefinition, TableConstraints},
        error::ApllodbErrorKind,
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
            match TableWideConstraints::new(&table_constraints, &column_definitions) {
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
            match TableWideConstraints::new(&table_constraints, &column_definitions) {
                Err(e) => match e.kind() {
                    ApllodbErrorKind::InvalidTableDefinition => {
                        println!("{}", e);
                    }
                    _ => panic!("unexpected error kind: {}", e),
                },
                Ok(tc) => panic!("Expected to be validation error: {:?}", tc),
            }
        }
    }
}