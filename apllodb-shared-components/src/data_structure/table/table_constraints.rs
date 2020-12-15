use crate::data_structure::{
    validation_helper::collection::find_dup_slow, ColumnName, TableConstraintKind,
};
use crate::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Table constraints.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct TableConstraints {
    kinds: Vec<TableConstraintKind>,
}

impl Default for TableConstraints {
    fn default() -> Self {
        Self { kinds: vec![] }
    }
}

impl TableConstraints {
    /// Constructor.
    ///
    /// # Failures
    /// - [InvalidTableDefinition](error/enum.ApllodbErrorKind.html#variant.InvalidTableDefinition) when:
    ///   - No [PrimaryKey](enum.TableWideConstraintKind.html#variant.PrimaryKey) is specified.
    ///   - Multiple [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey)s appear.
    ///   - More than 1 [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) /
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) constraints are applied to the same column set.
    ///   - Column sequence of [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) or
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) have duplicate column.
    pub fn new(constraints: Vec<TableConstraintKind>) -> ApllodbResult<Self> {
        Self::validate_col_duplication(&constraints)?;
        Self::validate_pk_existence(&constraints)?;
        Self::validate_multiple_pks(&constraints)?;
        Self::validate_pk_or_unique_to_same_cols(&constraints)?;
        Ok(Self { kinds: constraints })
    }

    fn validate_col_duplication(_constraints: &[TableConstraintKind]) -> ApllodbResult<()> {
        // TODO
        Ok(())
    }

    fn validate_pk_existence(constraints: &[TableConstraintKind]) -> ApllodbResult<()> {
        constraints
            .iter()
            .find_map(|table_constraint_kind| match table_constraint_kind {
                TableConstraintKind::PrimaryKey { .. } => Some(()),
                _ => None,
            })
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::InvalidTableDefinition,
                    "PRIMARY KEY is not specified",
                    None,
                )
            })
    }

    fn validate_multiple_pks(constraints: &[TableConstraintKind]) -> ApllodbResult<()> {
        if constraints
        .iter()
        .filter(|table_constraint_kind| matches!(table_constraint_kind, TableConstraintKind::PrimaryKey {..}))
        .count()
        > 1
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTableDefinition,
                "more than 1 PRIMARY KEY found",
                None,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_pk_or_unique_to_same_cols(
        constraints: &[TableConstraintKind],
    ) -> ApllodbResult<()> {
        let pk_unique_column_sets: Vec<HashSet<ColumnName>> = constraints
            .iter()
            .map(|table_constraint_kind| {
                let h: HashSet<ColumnName> = match table_constraint_kind {
                    TableConstraintKind::PrimaryKey { column_names }
                    | TableConstraintKind::Unique { column_names } => {
                        column_names.iter().cloned().collect()
                    }
                };
                h
            })
            .collect();

        if let Some(colset) = find_dup_slow(pk_unique_column_sets.iter()) {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidTableDefinition,
                format!(
                    "more than 1 PRIMARY KEY / UNIQUE are applied to the same column set: `{:?}`",
                    colset
                ),
                None,
            ))
        } else {
            Ok(())
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
    macro_rules! t_pk {
        ($($col_name: expr $(,)?)*) => {{
            $crate::data_structure::TableConstraintKind::PrimaryKey {
                column_names: vec![
                    $(
                        $crate::data_structure::ColumnName::new($col_name).unwrap(),
                    )*
                ],
            }

        }};
    }
    macro_rules! t_unique {
        ($($col_name: expr $(,)?)*) => {{
            $crate::data_structure::TableConstraintKind::Unique {
                column_names: vec![
                    $(
                        $crate::data_structure::ColumnName::new($col_name).unwrap(),
                    )*
                ],
            }
        }}
    }

    use super::TableConstraints;
    use crate::{data_structure::TableConstraintKind, error::ApllodbErrorKind};

    #[test]
    fn test_success() {
        let testset: Vec<Vec<TableConstraintKind>> = vec![
            vec![t_pk!("c1")],
            vec![t_pk!("c1"), t_unique!("c2")],
            vec![t_pk!("c1", "c2")],
            vec![t_pk!("c1"), t_unique!("c1", "c2")],
            vec![t_pk!("c1"), t_unique!("c1", "c2"), t_unique!("c2")],
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
            // no PK
            vec![t_unique!("c1")],
            // duplicate constraints.
            vec![t_pk!("c1"), t_pk!("c1")],
            vec![t_pk!("c0"), t_unique!("c1"), t_unique!("c1")],
            vec![
                t_pk!("c0"),
                t_unique!("c1"),
                t_unique!("c2"),
                t_unique!("c1"),
            ],
            vec![t_pk!("c1", "c2"), t_pk!("c1", "c2")],
            // PK & UNIQUE are applied to the same column sequence.
            vec![t_pk!("c1"), t_unique!("c1")],
            // Multiple PKs.
            vec![t_pk!("c1"), t_pk!("c2")],
        ];

        for constraints in testset {
            match TableConstraints::new(constraints) {
                Err(e) => match e.kind() {
                    ApllodbErrorKind::InvalidTableDefinition => {
                        println!("{:?}", e);
                    }
                    _ => panic!("unexpected error kind: {}", e),
                },
                Ok(tc) => panic!("Expected to be validation error: {:?}", tc),
            }
        }
    }
}
