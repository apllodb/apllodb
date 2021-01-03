pub(crate) mod boolean_expression;

use serde::{Deserialize, Serialize};

use self::boolean_expression::BooleanExpression;

use super::{column::column_name::ColumnName, value::sql_value::SqlValue};

/// Expression.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(SqlValue),

    /// Reference to column value
    ColumnNameVariant(ColumnName),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}

impl From<SqlValue> for Expression {
    fn from(sql_val: SqlValue) -> Self {
        Self::ConstantVariant(sql_val)
    }
}
