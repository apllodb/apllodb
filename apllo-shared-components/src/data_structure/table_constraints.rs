use super::{
    table_constraint_kind::TableConstraintKind, validation_helper::collection::find_dup, ColumnName,
};
use crate::error::{AplloError, AplloErrorKind, AplloResult};
use serde::{Deserialize, Serialize};

/// Table constraints.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct TableConstraints {
    kinds: Vec<TableConstraintKind>,
}
impl TableConstraints {
    /// Constructor.
    ///
    /// # Failures
    /// - [InvalidTableDefinition](error/enum.AplloErrorKind.html#variant.InvalidTableDefinition) when:
    ///   - Multiple [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey)s appear.
    ///   - `constraints` have duplicate element.
    ///   - [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) and
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) constraints have the same column sequence.
    ///   - Multiple [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey)s appear.
    ///
    /// TODO:
    /// - ColumnConstraints と合わせて見ないと、dup checkなので無意味
    /// - uniq性なのでcol seq で見ずに col set で見なきゃ
    pub fn new(constraints: Vec<TableConstraintKind>) -> AplloResult<Self> {
        if constraints
            .iter()
            .filter(|table_constraint_kind| match table_constraint_kind {
                TableConstraintKind::PrimaryKey { .. } => true,
                _ => false,
            })
            .count()
            > 1
        {
            Err(AplloError::new(
                AplloErrorKind::InvalidTableDefinition,
                "more than 1 PRIMARY KEY found",
                None,
            ))
        } else if let Some(kind) = find_dup(constraints.iter()) {
            Err(AplloError::new(
                AplloErrorKind::InvalidTableDefinition,
                format!("duplicate table constraint: `{:?}`", kind),
                None,
            ))
        } else {
            let pk_unique_column_seqs: Vec<&Vec<ColumnName>> = constraints
                .iter()
                .map(|table_constraint_kind| match table_constraint_kind {
                    TableConstraintKind::PrimaryKey { column_names } => column_names,
                    TableConstraintKind::Unique { column_names } => column_names,
                })
                .collect();

            if let Some(kind) = find_dup(pk_unique_column_seqs.iter()) {
                Err(AplloError::new(
                    AplloErrorKind::InvalidTableDefinition,
                    format!(
                        "PRIMARY KEY and UNIQUE are applied to the same column sequence: `{:?}`",
                        kind
                    ),
                    None,
                ))
            } else {
                Ok(Self { kinds: constraints })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TableConstraints;
    use crate::{
        data_structure::{
            column_name::ColumnName, table_constraint_kind::TableConstraintKind, ShortName,
        },
        error::AplloErrorKind,
    };

    macro_rules! pk {
        ($($col_name: expr $(,)?)*) => {
            TableConstraintKind::PrimaryKey {
                column_names: vec![
                    $(
                        ColumnName::from(ShortName::new($col_name).unwrap()),
                    )*
                ],
            }
        };
    }

    macro_rules! unique {
        ($($col_name: expr $(,)?)*) => {
            TableConstraintKind::Unique {
                column_names: vec![
                    $(
                        ColumnName::from(ShortName::new($col_name).unwrap()),
                    )*
                ],
            }
        };
    }

    #[test]
    fn test_success() {
        let testset: Vec<Vec<TableConstraintKind>> = vec![
            vec![],
            vec![pk!("c1")],
            vec![unique!("c1")],
            vec![pk!("c1", "c2")],
            vec![unique!("c1", "c2")],
            vec![pk!("c1"), unique!("c2")],
            vec![unique!("c1", "c2"), unique!("c2")],
            vec![unique!("c1"), unique!("c2"), unique!("c1")],
            vec![pk!("c1", "c2"), pk!("c1", "c2")],
            // PK & UNIQUE are applied to the same column sequence.
            vec![pk!("c1"), unique!("c1")],
            // Multiple PKs.
            vec![pk!("c1"), pk!("c2")],
        ];

        for constraints in testset {
            match TableConstraints::new(constraints) {
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

    #[test]
    fn test_failure_invalid_table_definition() {
        let testset: Vec<Vec<TableConstraintKind>> = vec![
            // duplicate constraints.
            vec![pk!("c1"), pk!("c1")],
            vec![unique!("c1"), unique!("c1")],
            vec![unique!("c1"), unique!("c2"), unique!("c1")],
            vec![pk!("c1", "c2"), pk!("c1", "c2")],
            // PK & UNIQUE are applied to the same column sequence.
            vec![pk!("c1"), unique!("c1")],
            // Multiple PKs.
            vec![pk!("c1"), pk!("c2")],
        ];

        for constraints in testset {
            match TableConstraints::new(constraints) {
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
