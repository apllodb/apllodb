use super::{
    table_constraint_kind::TableConstraintKind, validation_helper::collection::find_dup_slow,
    ColumnName,
};
use crate::error::{AplloError, AplloErrorKind, AplloResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    ///   - More than 1 [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) /
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) constraints are applied to the same column set.
    ///   - Column sequence of [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) or
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) have duplicate column.
    ///
    /// TODO: Check duplication with ColumnConstraints.
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
        } else {
            let pk_unique_column_sets: Vec<HashSet<&ColumnName>> = constraints
                .iter()
                .map(|table_constraint_kind| {
                    match table_constraint_kind {
                        TableConstraintKind::PrimaryKey { column_names } => column_names,
                        TableConstraintKind::Unique { column_names } => column_names,
                    }
                    .iter()
                    .collect()
                })
                .collect();

            if let Some(colset) = find_dup_slow(pk_unique_column_sets.iter()) {
                Err(AplloError::new(
                    AplloErrorKind::InvalidTableDefinition,
                    format!(
                        "more than 1 PRIMARY KEY / UNIQUE are applied to the same column set: `{:?}`",
                        colset
                    ),
                    None,
                ))
            } else {
                Ok(Self { kinds: constraints })
            }
        }
    }

    /// Ref to seq of [TableConstraintKind](enum.TableConstraintKind.html).
    pub fn kinds(&self) -> &[TableConstraintKind] {
        &self.kinds
    }
}

// TODO テストは書いているが、まだバリデーション条件考えきれてもいないし、いじめ方も足りないので、テストは全体的に見直す
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
        ];

        for constraints in testset {
            match TableConstraints::new(constraints) {
                Ok(_) => {}
                Err(e) => panic!("unexpected error kind: {}", e),
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

