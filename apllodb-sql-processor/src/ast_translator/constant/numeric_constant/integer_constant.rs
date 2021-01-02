use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, IntegerConstant};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// Translates into minimum-possible type.
    ///
    /// # Failures
    ///
    /// - [NumericValueOutOfRange](apllodb_shared_components::ApllodbErrorKind::NumericValueOutOfRange) when:
    ///   - `ast_integer_constant` is out of range of `i64`.
    pub(crate) fn integer_constant(
        ast_integer_constant: apllodb_ast::IntegerConstant,
    ) -> ApllodbResult<IntegerConstant> {
        let s = ast_integer_constant.0;

        s.parse::<i16>()
            .map(IntegerConstant::I16)
            .or_else(|_| s.parse::<i32>().map(IntegerConstant::I32))
            .or_else(|_| s.parse::<i64>().map(IntegerConstant::I64))
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

    use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult, IntegerConstant};
    use apllodb_sql_parser::apllodb_ast;

    use super::AstTranslator;

    #[test]
    fn test_ok() -> ApllodbResult<()> {
        struct TestDatum<'test> {
            input_ast_integer_constant: &'test str,
            expected_integer_constant: IntegerConstant,
        }

        let test_data: Vec<TestDatum<'_>> = vec![
            // I16
            TestDatum {
                input_ast_integer_constant: "0",
                expected_integer_constant: IntegerConstant::I16(0),
            },
            TestDatum {
                input_ast_integer_constant: "-1",
                expected_integer_constant: IntegerConstant::I16(-1),
            },
            TestDatum {
                input_ast_integer_constant: "32767",
                expected_integer_constant: IntegerConstant::I16(i16::MAX),
            },
            TestDatum {
                input_ast_integer_constant: "-32768",
                expected_integer_constant: IntegerConstant::I16(i16::MIN),
            },
            // I32
            TestDatum {
                input_ast_integer_constant: "32768",
                expected_integer_constant: IntegerConstant::I32((i16::MAX as i32) + 1),
            },
            TestDatum {
                input_ast_integer_constant: "-32769",
                expected_integer_constant: IntegerConstant::I32((i16::MIN as i32) - 1),
            },
            TestDatum {
                input_ast_integer_constant: "2147483647",
                expected_integer_constant: IntegerConstant::I32(i32::MAX),
            },
            TestDatum {
                input_ast_integer_constant: "-2147483648",
                expected_integer_constant: IntegerConstant::I32(i32::MIN),
            },
            // I64
            TestDatum {
                input_ast_integer_constant: "2147483648",
                expected_integer_constant: IntegerConstant::I64((i32::MAX as i64) + 1),
            },
            TestDatum {
                input_ast_integer_constant: "-2147483649",
                expected_integer_constant: IntegerConstant::I64((i32::MIN as i64) - 1),
            },
            TestDatum {
                input_ast_integer_constant: "9223372036854775807",
                expected_integer_constant: IntegerConstant::I64(i64::MAX),
            },
            TestDatum {
                input_ast_integer_constant: "-9223372036854775808",
                expected_integer_constant: IntegerConstant::I64(i64::MIN),
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing input `{:?}`...",
                test_datum.input_ast_integer_constant
            );
            let input_ast_integer_constant =
                apllodb_ast::IntegerConstant(test_datum.input_ast_integer_constant.to_string());
            let out_integer_constant = AstTranslator::integer_constant(input_ast_integer_constant)?;
            assert_eq!(out_integer_constant, test_datum.expected_integer_constant);
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
