pub(crate) mod boolean_expression;
pub(crate) mod constant;

use serde::{Deserialize, Serialize};

use self::{boolean_expression::BooleanExpression, constant::Constant};

use super::{column::column_name::ColumnName, value::sql_value::SqlValue};

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
        data_structure::{
            data_type::{data_type_kind::DataTypeKind, DataType},
            value::sql_value::SqlValue,
        },
        error::ApllodbResult,
        test_support::setup,
    };

    use super::{
        constant::{CharacterConstant, Constant, IntegerConstant, NumericConstant, TextConstant},
        Expression,
    };

    #[test]
    fn test_from_sql_value_into_expression() -> ApllodbResult<()> {
        setup();

        let input_expect_set: Vec<(SqlValue, Expression)> = vec![
            // NOT NULL
            (
                SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &i16::MAX)?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant::I16(i16::MAX)),
                )),
            ),
            (
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &i32::MAX)?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant::I32(i32::MAX)),
                )),
            ),
            (
                SqlValue::pack(&DataType::new(DataTypeKind::BigInt, false), &i64::MAX)?,
                Expression::ConstantVariant(Constant::NumericConstantVariant(
                    NumericConstant::IntegerConstantVariant(IntegerConstant::I64(i64::MAX)),
                )),
            ),
            (
                SqlValue::pack(
                    &DataType::new(DataTypeKind::Text, false),
                    &"abc".to_string(),
                )?,
                Expression::ConstantVariant(Constant::CharacterConstantVariant(
                    CharacterConstant::TextConstantVariant(TextConstant::new("abc".to_string())),
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
                    NumericConstant::IntegerConstantVariant(IntegerConstant::I16(i16::MAX)),
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
