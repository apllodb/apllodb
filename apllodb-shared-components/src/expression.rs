pub(crate) mod boolean_expression;
pub(crate) mod operator;

use serde::{Deserialize, Serialize};

use crate::{
    ApllodbResult, ComparisonFunction, LogicalFunction, NnSqlValue, SchemaIndex, SqlValue,
};

use self::{boolean_expression::BooleanExpression, operator::UnaryOperator};

/// Expression.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(SqlValue),

    /// Field/Column index of a record
    SchemaIndexVariant(SchemaIndex),

    /// With unary operator
    UnaryOperatorVariant(UnaryOperator, Box<Expression>),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}

impl Expression {
    /// Fully evaluate an expression to SqlValue.
    ///
    /// Row or Record is necessary for SchemaIndexVariant and it is supposed to be indirectly passed via `value_from_index`.
    fn to_sql_value(
        &self,
        value_from_index: &dyn Fn(&SchemaIndex) -> ApllodbResult<SqlValue>,
    ) -> ApllodbResult<SqlValue> {
        match self {
            Expression::ConstantVariant(sql_value) => Ok(sql_value.clone()),
            Expression::SchemaIndexVariant(idx) => value_from_index(idx),
            Expression::UnaryOperatorVariant(uni_op, child) => {
                let child_sql_value = child.to_sql_value(value_from_index)?;
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
                            let left_sql_value = left.to_sql_value(value_from_index)?;
                            let right_sql_value = right.to_sql_value(value_from_index)?;
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
                                    .to_sql_value(value_from_index)?;
                            let right_sql_value =
                                Expression::BooleanExpressionVariant(*(right.clone()))
                                    .to_sql_value(value_from_index)?;

                            let b = left_sql_value.to_bool()? && right_sql_value.to_bool()?;
                            Ok(SqlValue::NotNull(NnSqlValue::Boolean(b)))
                        }
                    }
                }
            },
        }
    }

    /// Fully evaluate an expression (including SchemaIndexVariant) to SqlValue.
    ///
    /// Row or Record is necessary for SchemaIndexVariant and it is supposed to be indirectly passed via `value_from_index`.
    pub fn to_sql_value_for_expr_with_index(
        &self,
        value_from_index: &dyn Fn(&SchemaIndex) -> ApllodbResult<SqlValue>,
    ) -> ApllodbResult<SqlValue> {
        self.to_sql_value(value_from_index)
    }

    /// Fully evaluate an expression (without SchemaIndexVariant) to SqlValue.
    ///
    /// # Panics
    ///
    /// if this expression includes at least 1 Expression::SchemaIndexVariant.
    pub fn to_sql_value_for_expr_without_index(&self) -> ApllodbResult<SqlValue> {
        self.to_sql_value(&|_| panic!("this expression includes SchemaIndexVariant so you must use to_sql_value_for_expr_with_index to get an SqlValue: {:#?}", self))
    }

    /// retrieves all SchemaIndex in a expression
    pub fn to_schema_indexes(&self) -> Vec<SchemaIndex> {
        fn helper_boolean_expr(boolean_expr: &BooleanExpression) -> Vec<SchemaIndex> {
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
                            let mut left = left.to_schema_indexes();
                            let mut right = right.to_schema_indexes();
                            left.append(&mut right);
                            left
                        }
                    }
                }
            }
        }

        match self {
            Expression::ConstantVariant(_) => vec![],
            Expression::SchemaIndexVariant(idx) => vec![idx.clone()],
            Expression::UnaryOperatorVariant(_op, expr) => expr.to_schema_indexes(),
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
    use crate::{
        ApllodbResult, BooleanExpression, Expression, SchemaIndex, SqlValue, UnaryOperator,
    };

    #[test]
    fn test_to_sql_value() -> ApllodbResult<()> {
        #[derive(new)]
        struct TestDatum {
            in_expr: Expression,
            in_value_from_index: Option<Box<dyn Fn(&SchemaIndex) -> ApllodbResult<SqlValue>>>,
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
            // SchemaIndex
            TestDatum::new(
                Expression::SchemaIndexVariant(SchemaIndex::from("x")),
                Some(Box::new(|_| Ok(SqlValue::factory_integer(1)))),
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
            let sql_value = match t.in_value_from_index {
                Some(value_from_index) => t
                    .in_expr
                    .to_sql_value_for_expr_with_index(value_from_index.as_ref())?,
                None => t.in_expr.to_sql_value_for_expr_without_index()?,
            };
            assert_eq!(sql_value, t.expected_sql_value);
        }

        Ok(())
    }
}
