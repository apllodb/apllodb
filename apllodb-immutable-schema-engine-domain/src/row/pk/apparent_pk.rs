use crate::row::column::pk_column::{
    column_data_type::PKColumnDataType, column_name::PKColumnName,
};
use apllodb_shared_components::{
    data_structure::{
        BooleanExpression, ColumnName, ComparisonFunction, Constant, Expression, LogicalFunction,
        SqlValue,
    },
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::PrimaryKey;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Primary key which other components than Storage Engine observes.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct ApparentPrimaryKey {
    pk_column_names: Vec<PKColumnName>,

    // real "key" of a record.
    sql_values: Vec<SqlValue>,
}

impl PrimaryKey for ApparentPrimaryKey {
    fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
        let target_cn = PKColumnName::from(column_name.clone());

        let target_sql_value = self
            .zipped()
            .iter()
            .find_map(|(cn, sql_value)| {
                if *cn == &target_cn {
                    Some(*sql_value)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("undefined column name in PK: `{}`", target_cn),
                    None,
                )
            })?;
        Ok(target_sql_value)
    }
}

impl ApparentPrimaryKey {
    pub fn column_names(&self) -> &[PKColumnName] {
        &self.pk_column_names
    }

    pub fn sql_values(&self) -> &[SqlValue] {
        &self.sql_values
    }

    pub fn zipped(&self) -> Vec<(&PKColumnName, &SqlValue)> {
        self.pk_column_names.iter().zip(&self.sql_values).collect()
    }

    pub fn column_data_types(&self) -> Vec<PKColumnDataType> {
        self.zipped()
            .into_iter()
            .map(|(cname, sql_value)| {
                PKColumnDataType::new(cname.clone(), sql_value.data_type().clone())
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
                    left: Box::new(Expression::ColumnNameVariant(
                        column_name.as_column_name().clone(),
                    )),
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
