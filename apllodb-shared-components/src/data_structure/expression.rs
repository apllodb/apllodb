mod boolean_expression;
mod constant;

pub use boolean_expression::{BooleanExpression, ComparisonFunction, LogicalFunction};
pub use constant::{CharacterConstant, Constant, IntegerConstant, NumericConstant, TextConstant};

use super::{ColumnName, SqlValue};
use serde::{Deserialize, Serialize};

/// Expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(Constant),

    /// Reference to column value
    ColumnNameVariant(ColumnName),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}

impl From<&SqlValue> for Expression {
    fn from(sql_val: &SqlValue) -> Self {
        let data_type = sql_val.data_type();

        if data_type.nullable() {
            // NULL?
            sql_val.unpack().unwrap_or_else(op)
        } else {}

        


        todo!()
    }
}

