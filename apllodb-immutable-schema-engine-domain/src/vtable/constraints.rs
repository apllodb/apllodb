use super::constraint_kind::TableWideConstraintKind;
use crate::row::column::pk_column::{
    column_data_type::PKColumnDataType, column_name::PKColumnName,
};
use apllodb_shared_components::{
    data_structure::{ColumnDefinition, TableConstraints},
    error::ApllodbResult,
};
use serde::{Deserialize, Serialize};

/// Table-wide constraints applied to record set.
///
/// Note that "table constraint" used mainly in syntax (`T_tableConstraint`) and "table-wide constraint" are different.
/// The former is "a constraint applied to the table itself or multiple columns". (e.g. FOREIGN KEY is a table constraint)
/// The latter is "a constraint applied to set of records". (e.g. FOREIGN KEY is NOT a table-wide constraint but a version constraint)
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct TableWideConstraints {
    kinds: Vec<TableWideConstraintKind>,
}
impl TableWideConstraints {
    /// Extract ApparentPrimaryKey column data types
    pub fn pk_column_data_types(&self) -> &[PKColumnDataType] {
        &self
            .kinds
            .iter()
            .find_map(|k| {
                if let TableWideConstraintKind::PrimaryKey { column_data_types } = k {
                    Some(column_data_types)
                } else {
                    None
                }
            })
            .expect("every table must have a primary key")
    }

    /// Extract ApparentPrimaryKey column names
    pub fn pk_column_names(&self) -> Vec<PKColumnName> {
        self.pk_column_data_types()
            .iter()
            .map(|cdt| cdt.column_name().clone())
            .collect()
    }

    /// Constructor that extracts Table constraints (set of record must obey)
    /// from TableConstraints and ColumnConstraints in each ColumnDefinition.
    ///
    /// # Failures
    ///
    /// - [InvalidTableDefinition](error/enum.ApllodbErrorKind.html#variant.InvalidTableDefinition) when:
    ///   - [PrimaryKey](enum.TableWideConstraintKind.html#variant.PrimaryKey) or
    ///     [Unique](enum.TableWideConstraintKind.html#variant.Unique) in `table_constraints` are applied to an unavailable column.
    pub(crate) fn new(
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<Self> {
        let kinds = table_constraints
            .kinds()
            .iter()
            .map(|tck| TableWideConstraintKind::new(column_definitions, tck))
            .collect::<ApllodbResult<Vec<TableWideConstraintKind>>>()?;

        Ok(Self { kinds })
    }
}

#[cfg(test)]
mod tests {
    use super::TableWideConstraints;
    use crate::test_support::setup;
    use apllodb_shared_components::{
        data_structure::{
            ColumnConstraints, ColumnDefinition, ColumnName, DataType, DataTypeKind,
            TableConstraintKind, TableConstraints,
        },
        error::{ApllodbErrorKind, ApllodbResult},
    };

    #[test]
    fn test_success() -> ApllodbResult<()> {
        setup();

        let c1_def = ColumnDefinition::new(
            ColumnName::new("c1")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::new(vec![])?,
        )?;
        let c2_def = ColumnDefinition::new(
            ColumnName::new("c2")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::new(vec![])?,
        )?;

        let testset: Vec<(TableConstraints, Vec<ColumnDefinition>)> = vec![
            (
                TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                    column_names: vec![c1_def.column_name().clone()],
                }])?,
                vec![c1_def.clone(), c2_def.clone()],
            ),
            (
                TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                    column_names: vec![c2_def.column_name().clone(), c1_def.column_name().clone()],
                }])?,
                vec![c1_def.clone(), c2_def.clone()],
            ),
            (
                TableConstraints::new(vec![
                    TableConstraintKind::PrimaryKey {
                        column_names: vec![c1_def.column_name().clone()],
                    },
                    TableConstraintKind::Unique {
                        column_names: vec![
                            c1_def.column_name().clone(),
                            c2_def.column_name().clone(),
                        ],
                    },
                ])?,
                vec![c1_def.clone(), c2_def.clone()],
            ),
        ];

        for (table_constraints, column_definitions) in testset {
            match TableWideConstraints::new(&table_constraints, &column_definitions) {
                Ok(_) => {}
                Err(e) => panic!("unexpected error kind: {}", e),
            }
        }

        Ok(())
    }

    #[test]
    fn test_failure_invalid_table_definition() -> ApllodbResult<()> {
        setup();

        let c1_def = ColumnDefinition::new(
            ColumnName::new("c1")?,
            DataType::new(DataTypeKind::Integer, false),
            ColumnConstraints::new(vec![])?,
        )?;

        let testset: Vec<(TableConstraints, Vec<ColumnDefinition>)> = vec![(
            TableConstraints::new(vec![TableConstraintKind::PrimaryKey {
                column_names: vec![ColumnName::new("c404")?],
            }])?,
            vec![c1_def],
        )];

        for (table_constraints, column_definitions) in testset {
            match TableWideConstraints::new(&table_constraints, &column_definitions) {
                Err(e) => match e.kind() {
                    ApllodbErrorKind::InvalidTableDefinition => {
                        println!("{:?}", e);
                    }
                    _ => panic!("unexpected error kind: {}", e),
                },
                Ok(tc) => panic!("Expected to be validation error: {:?}", tc),
            }
        }

        Ok(())
    }
}
