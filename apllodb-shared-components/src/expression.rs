pub(crate) mod boolean_expression;
pub(crate) mod operator;

use serde::{Deserialize, Serialize};

use crate::{
    record_schema::RecordSchema, ApllodbResult, ComparisonFunction, FieldIndex, FullFieldReference,
    LogicalFunction, NnSqlValue, Record, RecordFieldRefSchema, RecordIndex, SqlValue,
};

use self::{boolean_expression::BooleanExpression, operator::UnaryOperator};

/// Expression.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(SqlValue),

    /// Field/Column index of a record
    RecordIndexVariant(RecordIndex),

    /// With unary operator
    UnaryOperatorVariant(UnaryOperator, Box<Expression>),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}

impl Expression {
    /// # Panics
    ///
    /// if `record_for_index` is None for Expression::FullFieldReferenceVariant.
    pub fn to_sql_value(
        &self,
        record_for_index: Option<(&Record, &RecordSchema)>,
    ) -> ApllodbResult<SqlValue> {
        match self {
            Expression::ConstantVariant(sql_value) => Ok(sql_value.clone()),
            Expression::RecordIndexVariant(idx) => {
                let (record, schema) = record_for_index.expect(
                    "needs `record_for_field_ref` to eval Expression::FullFieldReferenceVariant",
                );
                let pos = schema.index(idx)?;
                record.get_sql_value(pos).map(|v| v.clone())
            }
            Expression::UnaryOperatorVariant(uni_op, child) => {
                let child_sql_value = child.to_sql_value(record_for_index)?;
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
                            let left_sql_value = left.to_sql_value(record_for_index)?;
                            let right_sql_value = right.to_sql_value(record_for_index)?;
                            left_sql_value
                                .sql_compare(&right_sql_value)
                                .map(|sql_compare_result| {
                                    SqlValue::NotNull(NnSqlValue::Boolean(
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
                                    .to_sql_value(record_for_index)?;
                            let right_sql_value =
                                Expression::BooleanExpressionVariant(*(right.clone()))
                                    .to_sql_value(record_for_index)?;

                            let b = left_sql_value.to_bool()? && right_sql_value.to_bool()?;
                            Ok(SqlValue::NotNull(NnSqlValue::Boolean(b)))
                        }
                    }
                }
            },
        }
    }

    /// retrieves all RecordIndex in a expression
    pub fn to_record_indexes(&self) -> Vec<RecordIndex> {
        fn helper_boolean_expr(boolean_expr: &BooleanExpression) -> Vec<RecordIndex> {
            match boolean_expr {
                BooleanExpression::LogicalFunctionVariant(logical_function) => {
                    match logical_function {
                        LogicalFunction::AndVariant { left, right } => {
                            let mut left = helper_boolean_expr(&*left);
                            let mut right = helper_boolean_expr(&*right);
                            left.append(&mut right);
                            left
                        }
                    }
                }
                BooleanExpression::ComparisonFunctionVariant(comparison_function) => {
                    match comparison_function {
                        ComparisonFunction::EqualVariant { left, right } => {
                            let mut left = left.to_record_indexes();
                            let mut right = right.to_record_indexes();
                            left.append(&mut right);
                            left
                        }
                    }
                }
            }
        }

        match self {
            Expression::ConstantVariant(_) => {
                vec![]
            }
            Expression::RecordIndexVariant(idx) => {
                vec![idx.clone()]
            }
            Expression::UnaryOperatorVariant(_op, expr) => expr.to_record_indexes(),
            Expression::BooleanExpressionVariant(bool_expr) => helper_boolean_expr(bool_expr),
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
    use crate::test_support::{fixture::*, test_models::People};
    use crate::{
        ApllodbResult, BooleanExpression, Expression, Record, RecordFieldRefSchema, SqlValue,
        UnaryOperator,
    };

    #[test]
    fn test_to_sql_value() -> ApllodbResult<()> {
        #[derive(Clone, Debug, new)]
        struct TestDatum {
            in_expr: Expression,
            in_record_for_field_ref: Option<(Record, RecordFieldRefSchema)>,
            expected_sql_value: SqlValue,
        }

        let test_data: Vec<TestDatum> = vec![
            // constants
            TestDatum::new(
                Expression::factory_integer(1),
                None,
                SqlValue::factory_integer(1),
            ),
            // unary op
            TestDatum::new(
                Expression::factory_uni_op(UnaryOperator::Minus, Expression::factory_integer(1)),
                None,
                SqlValue::factory_integer(-1),
            ),
            // FullFieldReference
            TestDatum::new(
                Expression::RecordIndexVariant(People::ffr_id()),
                Some((PEOPLE_RECORD1.clone(), People::schema())),
                SqlValue::factory_integer(1),
            ),
            // BooleanExpression
            TestDatum::new(
                Expression::factory_eq(Expression::factory_null(), Expression::factory_null()),
                None,
                SqlValue::factory_bool(false),
            ),
            TestDatum::new(
                Expression::factory_eq(
                    Expression::factory_integer(123),
                    Expression::factory_integer(123),
                ),
                None,
                SqlValue::factory_bool(true),
            ),
            TestDatum::new(
                Expression::factory_eq(
                    Expression::factory_integer(123),
                    Expression::factory_integer(-123),
                ),
                None,
                SqlValue::factory_bool(false),
            ),
            TestDatum::new(
                Expression::factory_and(
                    BooleanExpression::factory_eq(
                        Expression::factory_integer(123),
                        Expression::factory_integer(123),
                    ),
                    BooleanExpression::factory_eq(
                        Expression::factory_integer(456),
                        Expression::factory_integer(456),
                    ),
                ),
                None,
                SqlValue::factory_bool(true),
            ),
            TestDatum::new(
                Expression::factory_and(
                    BooleanExpression::factory_eq(
                        Expression::factory_integer(-123),
                        Expression::factory_integer(123),
                    ),
                    BooleanExpression::factory_eq(
                        Expression::factory_integer(456),
                        Expression::factory_integer(456),
                    ),
                ),
                None,
                SqlValue::factory_bool(false),
            ),
        ];

        for t in test_data {
            let sql_value = t
                .in_expr
                .to_sql_value(t.in_record_for_field_ref.as_ref().map(|(r, s)| (r, s)))?;
            assert_eq!(sql_value, t.expected_sql_value);
        }

        Ok(())
    }
}
