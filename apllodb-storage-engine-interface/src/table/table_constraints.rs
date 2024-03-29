use apllodb_shared_components::{find_dup_slow, ApllodbError, ApllodbResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::column::column_name::ColumnName;

use super::table_constraint_kind::TableConstraintKind;

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
    /// - [DdlError](apllodb_shared_components::SqlState::DdlError) when:
    ///   - No [PrimaryKey](crate::TableConstraintKind::PrimaryKey) is specified.
    ///   - Multiple [PrimaryKey](crate::TableConstraintKind::PrimaryKey)s appear.
    ///   - More than 1 [PrimaryKey](crate::TableConstraintKind::PrimaryKey) /
    ///     [Unique](crate::TableConstraintKind::Unique) constraints are applied to the same column set.
    ///   - Column sequence of [PrimaryKey](crate::TableConstraintKind::PrimaryKey) or
    ///     [Unique](crate::TableConstraintKind::Unique) have duplicate column.
    pub fn new(constraints: Vec<TableConstraintKind>) -> ApllodbResult<Self> {
        Self::validate_col_duplication(&constraints)?;
        Self::validate_pk_existence(&constraints)?;
        Self::validate_multiple_pks(&constraints)?;
        Self::validate_pk_or_unique_to_same_cols(&constraints)?;
        Ok(Self { kinds: constraints })
    }

    #[allow(clippy::unnecessary_wraps)]
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
            .ok_or_else(|| ApllodbError::ddl_error("PRIMARY KEY is not specified"))
    }

    fn validate_multiple_pks(constraints: &[TableConstraintKind]) -> ApllodbResult<()> {
        let pk_constraints_count = constraints
            .iter()
            .filter(|table_constraint_kind| {
                matches!(
                    table_constraint_kind,
                    TableConstraintKind::PrimaryKey { .. }
                )
            })
            .count();

        if pk_constraints_count > 1 {
            Err(ApllodbError::ddl_error("more than 1 PRIMARY KEY found"))
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
            Err(ApllodbError::ddl_error(format!(
                "more than 1 PRIMARY KEY / UNIQUE are applied to the same column set: `{:?}`",
                colset
            )))
        } else {
            Ok(())
        }
    }

    /// Ref to seq of [TableConstraintKind](enum.TableConstraintKind.html).
    pub fn kinds(&self) -> &[TableConstraintKind] {
        &self.kinds
    }
}

// TODO Needs more consideration about tests
// Validation specification isn't fixed.
#[cfg(test)]
mod tests {
    macro_rules! t_pk {
        ($($col_name: expr $(,)?)*) => {{
            $crate::TableConstraintKind::PrimaryKey {
                column_names: vec![
                    $(
                        $crate::ColumnName::new($col_name).unwrap(),
                    )*
                ],
            }

        }};
    }
    macro_rules! t_unique {
        ($($col_name: expr $(,)?)*) => {{
            $crate::TableConstraintKind::Unique {
                column_names: vec![
                    $(
                        $crate::ColumnName::new($col_name).unwrap(),
                    )*
                ],
            }
        }}
    }

    use super::{super::table_constraint_kind::TableConstraintKind, TableConstraints};
    use apllodb_shared_components::SqlState;

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
                    SqlState::DdlError => {
                        println!("{:?}", e);
                    }
                    _ => panic!("unexpected error kind: {}", e),
                },
                Ok(tc) => panic!("Expected to be validation error: {:?}", tc),
            }
        }
    }
}
