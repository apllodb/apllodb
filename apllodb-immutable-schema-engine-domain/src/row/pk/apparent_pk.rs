use apllodb_shared_components::{
    data_structure::{
        BooleanExpression, ColumnDataType, ColumnName, ComparisonFunction, Constant, Expression,
        LogicalFunction, SqlValue,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::PrimaryKey;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    column_names: ApparentPrimaryKeyColumnNames,

    // real "key" of a record.
    sql_values: Vec<SqlValue>,
}

/// ApparentPrimaryKey without values.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKeyColumnNames(Vec<ColumnName>);
impl ApparentPrimaryKeyColumnNames {
    pub fn column_names(&self) -> &[ColumnName] {
        &self.0
    }
}

impl PrimaryKey for ApparentPrimaryKey {
    fn column_names(&self) -> &[ColumnName] {
        &self.column_names.0
    }
}

impl ApparentPrimaryKey {
    pub fn sql_values(&self) -> &[SqlValue] {
        &self.sql_values
    }

    pub fn zipped(&self) -> Vec<(&ColumnName, &SqlValue)> {
        self.column_names.0.iter().zip(&self.sql_values).collect()
    }

    pub fn column_data_types(&self) -> Vec<ColumnDataType> {
        self.zipped()
            .into_iter()
            .map(|(cname, sql_value)| {
                ColumnDataType::new(cname.clone(), sql_value.data_type().clone())
            })
            .collect()
    }

    pub fn to_condition_expression(&self) -> ApllodbResult<BooleanExpression> {
        let mut comparisons = self
            .zipped()
            .into_iter()
            .map(|(column_name, sql_value)| {
                let constant_expr = Constant::from(sql_value);
                ComparisonFunction::EqualVariant {
                    left: Box::new(Expression::ColumnNameVariant(column_name.clone())),
                    right: Box::new(Expression::ConstantVariant(constant_expr)),
                }
            })
            .collect::<VecDeque<ComparisonFunction>>();

        let mut boolean_expr = BooleanExpression::ComparisonFunctionVariant(
            comparisons
                .pop_front()
                .expect("ApparentPrimaryKey has at least 1 column value"),
        );
        while let Some(comparison) = comparisons.pop_front() {
            boolean_expr = BooleanExpression::LogicalFunctionVariant(LogicalFunction::AndVariant {
                left: Box::new(boolean_expr),
                right: Box::new(BooleanExpression::ComparisonFunctionVariant(comparison)),
            });
        }

        Ok(boolean_expr)
    }
}
