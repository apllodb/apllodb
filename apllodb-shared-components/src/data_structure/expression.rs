pub(crate) mod boolean_expression;
pub(crate) mod operator;

use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::{ApllodbError, ApllodbErrorKind, ApllodbResult};

use self::{boolean_expression::BooleanExpression, operator::UnaryOperator};

use super::{column::column_name::ColumnName, value::sql_value::SqlValue};

/// Expression.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(SqlValue),

    /// Reference to column value
    ColumnNameVariant(ColumnName),

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
    /// - [DataException](apllodb_shared_components::ApllodbErrorKind::DataException) when:
    ///   - expression cannot be folded into an SqlValue
    fn try_from(expression: Expression) -> ApllodbResult<Self> {
        match expression {
            Expression::ConstantVariant(sql_value) => Ok(sql_value),
            Expression::ColumnNameVariant(c) => Err(ApllodbError::new(
                ApllodbErrorKind::DataException,
                format!("column name `{}` cannot folded into SqlValue", c.as_str()),
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
