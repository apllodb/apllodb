use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, NnSqlValue, SqlValue,
};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// # Failures
    ///
    /// - [NumericValueOutOfRange](apllodb_shared_components::ApllodbErrorKind::NumericValueOutOfRange) when:
    ///   - `ast_integer_constant` is out of range of `i64`.
    pub(crate) fn integer_constant(
        ast_integer_constant: apllodb_ast::IntegerConstant,
    ) -> ApllodbResult<SqlValue> {
        let s = ast_integer_constant.0;

        s.parse::<i16>()
            .map(|i| SqlValue::NotNull(NnSqlValue::SmallInt(i)))
            .or_else(|_| {
                s.parse::<i32>()
                    .map(|i| SqlValue::NotNull(NnSqlValue::Integer(i)))
            })
            .or_else(|_| {
                s.parse::<i64>()
                    .map(|i| SqlValue::NotNull(NnSqlValue::BigInt(i)))
            })
            .map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::NumericValueOutOfRange,
                    format!(
                        "integer value `{}` could not be parsed as i64 (max supported size)",
                        s
                    ),
                    Some(Box::new(e)),
                )
            })
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use apllodb_shared_components::{
        ApllodbErrorKind, ApllodbResult, NnSqlValue, SqlType, SqlValue,
    };
    use apllodb_sql_parser::apllodb_ast;

    use super::AstTranslator;

    #[test]
    fn test_ok() -> ApllodbResult<()> {
        struct TestDatum<'test> {
            input_ast_integer_constant: &'test str,
            expected_sql_type: SqlType,
            expected_rust_value: i64,
        }

        let test_data: Vec<TestDatum<'_>> = vec![
            // I16
            TestDatum {
                input_ast_integer_constant: "0",
                expected_sql_type: SqlType::small_int(),
                expected_rust_value: 0,
            },
            TestDatum {
                input_ast_integer_constant: "-1",
                expected_sql_type: SqlType::small_int(),
                expected_rust_value: -1,
            },
            TestDatum {
                input_ast_integer_constant: "32767",
                expected_sql_type: SqlType::small_int(),
                expected_rust_value: i16::MAX as i64,
            },
            TestDatum {
                input_ast_integer_constant: "-32768",
                expected_sql_type: SqlType::small_int(),
                expected_rust_value: i16::MIN as i64,
            },
            // I32
            TestDatum {
                input_ast_integer_constant: "32768",
                expected_sql_type: SqlType::integer(),
                expected_rust_value: (i16::MAX as i64) + 1,
            },
            TestDatum {
                input_ast_integer_constant: "-32769",
                expected_sql_type: SqlType::integer(),
                expected_rust_value: (i16::MIN as i64) - 1,
            },
            TestDatum {
                input_ast_integer_constant: "2147483647",
                expected_sql_type: SqlType::integer(),
                expected_rust_value: i32::MAX as i64,
            },
            TestDatum {
                input_ast_integer_constant: "-2147483648",
                expected_sql_type: SqlType::integer(),
                expected_rust_value: i32::MIN as i64,
            },
            // I64
            TestDatum {
                input_ast_integer_constant: "2147483648",
                expected_sql_type: SqlType::big_int(),
                expected_rust_value: (i32::MAX as i64) + 1,
            },
            TestDatum {
                input_ast_integer_constant: "-2147483649",
                expected_sql_type: SqlType::big_int(),
                expected_rust_value: (i32::MIN as i64) - 1,
            },
            TestDatum {
                input_ast_integer_constant: "9223372036854775807",
                expected_sql_type: SqlType::big_int(),
                expected_rust_value: i64::MAX,
            },
            TestDatum {
                input_ast_integer_constant: "-9223372036854775808",
                expected_sql_type: SqlType::big_int(),
                expected_rust_value: i64::MIN,
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing input `{:?}`...",
                test_datum.input_ast_integer_constant
            );
            let input_ast_integer_constant =
                apllodb_ast::IntegerConstant(test_datum.input_ast_integer_constant.to_string());
            if let SqlValue::NotNull(out_sql_value) =
                AstTranslator::integer_constant(input_ast_integer_constant)?
            {
                assert_eq!(out_sql_value.sql_type(), test_datum.expected_sql_type);
                assert_eq!(
                    out_sql_value,
                    NnSqlValue::BigInt(test_datum.expected_rust_value)
                );
            } else {
                unreachable!()
            }
        }

        Ok(())
    }

    #[test]
    fn test_err() {
        let test_data: Vec<&str> = vec!["9223372036854775808", "-9223372036854775809"];

        for test_datum in test_data {
            log::debug!("testing input `{:?}`...", test_datum);
            let input_ast_integer_constant = apllodb_ast::IntegerConstant(test_datum.to_string());
            assert_eq!(
                *AstTranslator::integer_constant(input_ast_integer_constant)
                    .unwrap_err()
                    .kind(),
                ApllodbErrorKind::NumericValueOutOfRange
            );
        }
    }
}
