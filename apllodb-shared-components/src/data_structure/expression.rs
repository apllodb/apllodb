pub(crate) mod boolean_expression;
pub(crate) mod operator;

use serde::{Deserialize, Serialize};

use crate::{
    ApllodbResult, ComparisonFunction, FieldIndex, FullFieldReference, LogicalFunction, NNSqlValue,
    Record, RecordFieldRefSchema,
};

use self::{boolean_expression::BooleanExpression, operator::UnaryOperator};

use super::value::sql_value::SqlValue;

/// Expression.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(SqlValue),

    /// Reference to field
    FullFieldReferenceVariant(FullFieldReference),

    /// With unary operator
    UnaryOperatorVariant(UnaryOperator, Box<Expression>),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}

impl Expression {
    pub fn to_sql_value(
        &self,
        record: &Record,
        schema: &RecordFieldRefSchema,
    ) -> ApllodbResult<SqlValue> {
        match self {
            Expression::ConstantVariant(sql_value) => Ok(sql_value.clone()),
            Expression::FullFieldReferenceVariant(ffr) => {
                let idx = schema.resolve_index(&FieldIndex::from(ffr.clone()))?;
                record.get_sql_value(idx).map(|v| v.clone())
            }
            Expression::UnaryOperatorVariant(uni_op, child) => {
                let child_sql_value = child.to_sql_value(record, schema)?;
                match (uni_op, child_sql_value) {
                    (UnaryOperator::Minus, SqlValue::Null) => Ok(SqlValue::Null),
                    (UnaryOperator::Minus, SqlValue::NotNull(nn_sql_value)) => {
                        Ok(SqlValue::NotNull(nn_sql_value.negate()?))
                    }
                }
            }
            Expression::BooleanExpressionVariant(bool_expr) => match bool_expr {
                BooleanExpression::ComparisonFunctionVariant(comparison_function) => {
                    match comparison_function {
                        ComparisonFunction::EqualVariant { left, right } => {
                            let left_sql_value = left.to_sql_value(record, schema)?;
                            let right_sql_value = right.to_sql_value(record, schema)?;
                            left_sql_value
                                .sql_compare(&right_sql_value)
                                .map(|sql_compare_result| {
                                    SqlValue::NotNull(NNSqlValue::Boolean(
                                        sql_compare_result.is_equal(),
                                    ))
                                })
                        }
                    }
                }
                BooleanExpression::LogicalFunctionVariant(logical_function) => {
                    match logical_function {
                        LogicalFunction::AndVariant { left, right } => {
                            let left_sql_value =
                                Expression::BooleanExpressionVariant(*(left.clone()))
                                    .to_sql_value(record, schema)?;
                            let right_sql_value =
                                Expression::BooleanExpressionVariant(*(right.clone()))
                                    .to_sql_value(record, schema)?;

                            let b = left_sql_value.to_bool()? && right_sql_value.to_bool()?;
                            Ok(SqlValue::NotNull(NNSqlValue::Boolean(b)))
                        }
                    }
                }
            },
        }
    }
}

impl From<SqlValue> for Expression {
    fn from(sql_val: SqlValue) -> Self {
        Self::ConstantVariant(sql_val)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{ApllodbResult, Expression, SqlValue, UnaryOperator};

    #[test]
    fn test_try_from_success() -> ApllodbResult<()> {
        let expr_vs_expected_sql_value: Vec<(Expression, SqlValue)> = vec![
            (Expression::factory_integer(1), SqlValue::factory_integer(1)),
            (
                Expression::factory_uni_op(UnaryOperator::Minus, Expression::factory_integer(1)),
                SqlValue::factory_integer(-1),
            ),
        ];

        for (expr, expected_sql_value) in expr_vs_expected_sql_value {
            let sql_value = SqlValue::try_from(expr)?;
            assert_eq!(sql_value, expected_sql_value);
        }

        Ok(())
    }
}
