pub(crate) mod field_index;

use crate::{error::ApllodbResult, traits::sql_convertible::SqlConvertible, SqlValues};
use std::ops::Index;

use super::value::sql_value::SqlValue;

/// Record representation used in client and query processor.
/// Storage engine uses Row, which does not treat `Expression`s but only does `ColumnName`.
///
/// Record is meant to be read-only data.
/// It is created while SELECT by a storage engine or query processor.
#[derive(Clone, PartialEq, Debug)]
pub struct Record {
    values: SqlValues,
}

impl Record {
    /// Constructor
    pub fn new(values: SqlValues) -> Self {
        Self { values }
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
    pub fn get<T: SqlConvertible>(&self, index: usize) -> ApllodbResult<Option<T>> {
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
    pub fn get_sql_value(&self, index: usize) -> ApllodbResult<&SqlValue> {
        let sql_value = self.values.index(index);
        Ok(sql_value)
    }

    /// Shrink a record into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(mut self, indexes: &[usize]) -> ApllodbResult<Self> {
        self.values = self.values.projection(&indexes);
        Ok(self)
    }

    /// Joins another record after this record.
    pub fn join(mut self, right: Self) -> Self {
        self.values.join(right.values);
        self
    }

    /// Get raw representation
    pub fn into_values(self) -> SqlValues {
        self.values
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
            // FullFieldReference
            TestDatum {
                in_record: T_PEOPLE_R1.clone(),
                in_condition: Expression::FullFieldReferenceVariant(People::ffr_id()),
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
