use super::table_constraint_kind::TableConstraintKind;
use crate::error::AplloResult;
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
    ///   - `constraints` have duplicate element.
    ///   - [PrimaryKey](enum.TableConstraintKind.html#variant.PrimaryKey) and
    ///     [Unique](enum.TableConstraintKind.html#variant.Unique) constraints have the same column sequence.
    pub fn new(constraints: Vec<TableConstraintKind>) -> AplloResult<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::TableConstraints;
    use crate::data_structure::{
        column_name::ColumnName, table_constraint_kind::TableConstraintKind,
    };

    #[test]
    fn test_failure_invalid_table_definition() {
        let constraints = vec![
            TableConstraintKind::PrimaryKey {
                column_names: vec![ColumnName::new("c1")],
            },
            TableConstraintKind::PrimaryKey {
                column_names: vec![ColumnName::new("c1")],
            },
        ];

        match TableConstraints::new(constraints) {
            Err(e) => match e.kind() {
                AplloErrorKind::InvalidTableDefinition => {}
                _ => panic!("Unexpected error kind: {}", e),
            },
            Ok(tc) => panic!("Expected to be validation error: {:?}", tc),
        }
    }
}
