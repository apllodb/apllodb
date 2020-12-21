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
        let constant = Constant::from(sql_val);
        Self::ConstantVariant(constant)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_structure::{DataType, DataTypeKind, SqlValue},
        error::ApllodbResult,
        test_support::setup,
    };

    use super::{
        CharacterConstant, Constant, Expression, IntegerConstant, NumericConstant, TextConstant,
    };

    #[test]
    fn test_from_sql_value_into_expression() -> ApllodbResult<()> {
        setup();

        let input_expect_set: Vec<(SqlValue, Expression)> = vec![
            // NOT NULL
            (
                SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &i16::MAX)?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant(i16::MAX as i64)),
                )),
            ),
            (
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &i32::MAX)?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant(i32::MAX as i64)),
                )),
            ),
            (
                SqlValue::pack(&DataType::new(DataTypeKind::BigInt, false), &i64::MAX)?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant(i64::MAX)),
                )),
            ),
            (
                SqlValue::pack(
                    &DataType::new(DataTypeKind::Text, false),
                    &"abc".to_string(),
                )?,
                Expression::ConstantVariant(Constant::CharacterConstantVariant(
                    CharacterConstant::TextConstantVariant(TextConstant("abc".to_string())),
                )),
            ),
            // NULLABLE, IS NULL
            (
                SqlValue::null(),
                Expression::ConstantVariant(Constant::Null),
            ),
            // NULLABLE, IS NOT NULL
            (
                SqlValue::pack(
                    &DataType::new(DataTypeKind::SmallInt, true),
                    &Some(i16::MAX),
                )?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant(i16::MAX as i64)),
                )),
            ),
        ];

        for (input, expect) in input_expect_set {
            let output = Expression::from(&input);
            assert_eq!(output, expect);
        }

        Ok(())
    }
}
