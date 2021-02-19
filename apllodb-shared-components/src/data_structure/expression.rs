pub(crate) mod boolean_expression;
pub(crate) mod operator;

use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::{ApllodbError, ApllodbErrorKind, ApllodbResult, SelectFieldReference};

use self::{boolean_expression::BooleanExpression, operator::UnaryOperator};

use super::value::sql_value::SqlValue;

/// Expression.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(SqlValue),

    /// Reference to field
    SelectFieldReferenceVariant(SelectFieldReference),

    /// With unary operator
    UnaryOperatorVariant(UnaryOperator, Box<Expression>),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}

impl From<SqlValue> for Expression {
    fn from(sql_val: SqlValue) -> Self {
        Self::ConstantVariant(sql_val)
    }
}

impl TryFrom<Expression> for SqlValue {
    type Error = ApllodbError;

    /// # Failures
    ///
    /// - [DataException](crate::ApllodbErrorKind::DataException) when:
    ///   - expression cannot be folded into an SqlValue
    fn try_from(expression: Expression) -> ApllodbResult<Self> {
        match expression {
            Expression::ConstantVariant(sql_value) => Ok(sql_value),
            Expression::SelectFieldReferenceVariant(ffr) => Err(ApllodbError::new(
                ApllodbErrorKind::DataException,
                format!("field `{}` cannot be into SqlValue", ffr),
                None,
            )),
            Expression::UnaryOperatorVariant(uni_op, child) => {
                let child_sql_value = SqlValue::try_from(*child)?;
                match (uni_op, child_sql_value) {
                    (UnaryOperator::Minus, SqlValue::Null) => Ok(SqlValue::Null),
                    (UnaryOperator::Minus, SqlValue::NotNull(nn_sql_value)) => {
                        Ok(SqlValue::NotNull(nn_sql_value.negate()?))
                    }
                }
            }
            Expression::BooleanExpressionVariant(_) => {
                unimplemented!()
            }
        }
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
