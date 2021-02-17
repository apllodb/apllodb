pub(crate) mod field_index;

use crate::{
    error::ApllodbResult, traits::sql_convertible::SqlConvertible, ApllodbError, ApllodbErrorKind,
    BooleanExpression, ComparisonFunction, Expression, FieldIndex, FullFieldReference,
    LogicalFunction, SqlValues, UnaryOperator,
};
use std::{ops::Index, sync::Arc};

use super::{
    record_iterator::record_field_ref_schema::RecordFieldRefSchema, value::sql_value::SqlValue,
};

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
///
/// Record is meant to be read-only data.
/// It is created while SELECT by a storage engine or query processor.
#[derive(Clone, PartialEq, Debug)]
pub struct Record {
    schema: Arc<RecordFieldRefSchema>,
    values: SqlValues,
}

impl Record {
    /// Constructor
    pub fn new(schema: Arc<RecordFieldRefSchema>, values: SqlValues) -> Self {
        Self { schema, values }
    }

    /// Get Rust value from record's field.
    ///
    /// Returns `None` if matching [SqlValue](crate::SqlValue) is NULL.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    /// - Errors from [SqlValue::unpack()](x.html).
    pub fn get<T: SqlConvertible>(&self, index: &FieldIndex) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(index)?;
        let ret = match sql_value {
            SqlValue::Null => None,
            SqlValue::NotNull(nn) => Some(nn.unpack()?),
        };
        Ok(ret)
    }

    /// Get [SqlValue](crate::SqlValue) from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get_sql_value(&self, index: &FieldIndex) -> ApllodbResult<&SqlValue> {
        let idx = self.schema.resolve_index(index)?;
        let sql_value = self.values.index(idx);
        Ok(sql_value)
    }

    /// Shrink a record into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(mut self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let idxs: Vec<usize> = projection
            .iter()
            .map(|index| self.schema.resolve_index(index))
            .collect::<ApllodbResult<_>>()?;

        self.schema = Arc::new(self.schema.projection(projection)?);
        self.values = self.values.projection(&idxs);

        Ok(self)
    }

    /// Check if whether this record satisfies selection condition.
    ///
    /// # Failures
    ///
    /// - [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - `condition` is not evaluated as BOOLEAN type.
    pub fn selection(&self, condition: &Expression) -> ApllodbResult<bool> {
        match condition {
            Expression::ConstantVariant(_) | Expression::UnresolvedFieldReferenceVariant(_) => {
                let sql_value = self.eval_to_sql_value(condition)?;
                self.selection_sql_value(&sql_value)
            }
            Expression::UnaryOperatorVariant(_, _) => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                "(unary_op)(Expression) cannot be evaluated as BOOLEAN",
                None,
            )),
            Expression::BooleanExpressionVariant(boolean_expr) => {
                self.selection_boolean_expression(boolean_expr)
            }
        }
    }

    fn selection_sql_value(&self, sql_value: &SqlValue) -> ApllodbResult<bool> {
        match sql_value {
            SqlValue::Null => Ok(false), // NULL is always evaluated as FALSE
            SqlValue::NotNull(nn_sql_value) => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "{:?} cannot be evaluated as BOOLEAN",
                    nn_sql_value.sql_type()
                ),
                None,
            )),
        }
    }

    fn selection_boolean_expression(
        &self,
        boolean_expr: &BooleanExpression,
    ) -> ApllodbResult<bool> {
        match boolean_expr {
            BooleanExpression::ComparisonFunctionVariant(comparison_function) => {
                match comparison_function {
                    ComparisonFunction::EqualVariant { left, right } => {
                        let left_sql_value = self.eval_to_sql_value(left)?;
                        let right_sql_value = self.eval_to_sql_value(right)?;
                        Ok(left_sql_value == right_sql_value)
                    }
                }
            }
            BooleanExpression::LogicalFunctionVariant(logical_function) => match logical_function {
                LogicalFunction::AndVariant { left, right } => {
                    let left_b = self.selection_boolean_expression(left)?;
                    let right_b = self.selection_boolean_expression(right)?;
                    Ok(left_b && right_b)
                }
            },
        }
    }

    /// Joins another record after this record.
    pub fn join(mut self, right: Self) -> Self {
        self.schema = Arc::new(self.schema.joined(right.schema()));
        self.values.join(right.values);
        self
    }

    /// Get raw representation
    pub fn into_values(self) -> SqlValues {
        self.values
    }

    /// Get raw representation
    pub fn into_ffr_vals(self) -> Vec<(FullFieldReference, SqlValue)> {
        self.schema
            .as_full_field_references()
            .iter()
            .cloned()
            .zip(self.values)
            .collect()
    }

    /// ref to schema
    pub fn schema(&self) -> &RecordFieldRefSchema {
        self.schema.as_ref()
    }

    /// # Failures
    ///
    /// - [DatatypeMismatch](crate::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - failed to evaluate as SqlValue.
    ///
    /// TODO no clone()
    fn eval_to_sql_value(&self, expr: &Expression) -> ApllodbResult<SqlValue> {
        match expr {
            Expression::ConstantVariant(sql_value) => Ok(sql_value.clone()),
            Expression::UnresolvedFieldReferenceVariant(ufr) => {
                let index = FieldIndex::from(ufr.clone());
                let sql_value = self.get_sql_value(&index)?;
                Ok(sql_value.clone())
            }
            Expression::UnaryOperatorVariant(op, expr) => {
                let sql_value = self.eval_to_sql_value(expr)?;
                match op {
                    UnaryOperator::Minus => match sql_value {
                        SqlValue::Null => Ok(sql_value),
                        SqlValue::NotNull(nn_sql_value) => {
                            Ok(SqlValue::NotNull(nn_sql_value.negate()?))
                        }
                    },
                }
            }
            Expression::BooleanExpressionVariant(_) => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                "BooleanExpression cannot be evaluated as SqlValue",
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_support::{fixture::T_PEOPLE_R1, test_models::People},
        ApllodbErrorKind, BooleanExpression, Expression, Record,
    };

    #[test]
    fn test_selection() {
        struct TestDatum {
            in_record: Record,
            in_condition: Expression,
            expected_result: Result<bool, ApllodbErrorKind>,
        }

        let test_data: Vec<TestDatum> = vec![
            // constants
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::factory_null(),
                expected_result: Ok(false),
            },
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::factory_integer(123),
                expected_result: Err(ApllodbErrorKind::DatatypeMismatch),
            },
            // UnresolvedFieldReference
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::UnresolvedFieldReferenceVariant(People::ufr_id()),
                expected_result: Err(ApllodbErrorKind::DatatypeMismatch),
            },
            // BooleanExpression
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::factory_eq(
                    Expression::factory_null(),
                    Expression::factory_null(),
                ),
                expected_result: Ok(false),
            },
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::factory_eq(
                    Expression::factory_integer(123),
                    Expression::factory_integer(123),
                ),
                expected_result: Ok(true),
            },
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::factory_eq(
                    Expression::factory_integer(123),
                    Expression::factory_integer(-123),
                ),
                expected_result: Ok(false),
            },
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::factory_and(
                    BooleanExpression::factory_eq(
                        Expression::factory_integer(123),
                        Expression::factory_integer(123),
                    ),
                    BooleanExpression::factory_eq(
                        Expression::factory_integer(456),
                        Expression::factory_integer(456),
                    ),
                ),
                expected_result: Ok(true),
            },
        ];

        for test_datum in test_data {
            let result = test_datum.in_record.selection(&test_datum.in_condition);
            match (result, test_datum.expected_result) {
                (Ok(b), Ok(b_expected)) => assert_eq!(b, b_expected),
                (Err(e), Err(e_expected)) => assert_eq!(e.kind(), &e_expected),
                (r, r_expected) => panic!("expected: {:#?}, got: {:#?}", r_expected, r),
            }
        }
    }
}
