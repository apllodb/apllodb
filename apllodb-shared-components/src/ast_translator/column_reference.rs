use crate::{
    data_structure::reference::{
        correlation_reference::CorrelationReference, field_reference::FieldReference,
    },
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, FullFieldReference,
};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// TODO may need Catalog value when:
    /// - ast_column_reference does not have correlation and
    /// - ast_from_items are more than 1
    /// because this function has to determine which of `from1` or `from2` `field1` is from.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](crate::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `correlations` is empty.
    /// - [UndefinedColumn](crate::ApllodbErrorKind::UndefinedColumn) when:
    ///   - none of `correlations` has field named `ast_column_reference.column_name`
    /// - [UndefinedObject](crate::ApllodbErrorKind::UndefinedObject) when:
    ///   - `ast_column_reference` has a correlation but it is not any of `correlations`.
    pub fn column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<FullFieldReference> {
        if correlations.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidColumnReference,
                format!(
                    "no FROM item. cannot detect where `{:?}` field is from",
                    ast_column_reference
                ),
                None,
            ))
        } else {
            if let Some(ast_corr) = ast_column_reference.correlation {
                Self::column_reference_with_corr(
                    ast_corr,
                    ast_column_reference.column_name,
                    correlations,
                )
            } else {
                Self::column_reference_without_corr(ast_column_reference.column_name, correlations)
            }
        }
    }

    fn column_reference_with_corr(
        _ast_correlation: apllodb_ast::Correlation,
        _ast_column_name: apllodb_ast::ColumnName,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<FullFieldReference> {
        assert!(!correlations.is_empty());

        todo!()
    }

    fn column_reference_without_corr(
        ast_column_name: apllodb_ast::ColumnName,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<FullFieldReference> {
        assert!(!correlations.is_empty());
        if correlations.len() > 1 {
            unimplemented!(
                "needs catalog info to detect which table has the column `{:?}`",
                ast_column_name
            )
        }

        // SELECT C FROM T (AS a)?;
        // -> C is from T
        let correlation_reference = correlations[0].clone();

        let field_reference =
            FieldReference::ColumnNameVariant(ColumnName::new(ast_column_name.0 .0)?);

        Ok(FullFieldReference::new(
            correlation_reference,
            field_reference,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ApllodbErrorKind, ApllodbResult, AstTranslator, CorrelationReference, FullFieldReference,
    };
    use apllodb_sql_parser::apllodb_ast::{ColumnReference, Correlation};
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ast_column_reference: ColumnReference,
        correlations: Vec<CorrelationReference>,
        expected_result: Result<FullFieldReference, ApllodbErrorKind>,
    }

    #[test]
    fn test_column_reference() -> ApllodbResult<()> {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![],
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![],
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![CorrelationReference::factory_tn("t")],
                Ok(FullFieldReference::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![CorrelationReference::factory_tn("t")],
                Ok(FullFieldReference::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t1")), "c"),
                vec![CorrelationReference::factory_tn("t2")],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![CorrelationReference::factory_ta("t", "a")],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![CorrelationReference::factory_ta("t", "a")],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("a")), "c"),
                vec![CorrelationReference::factory_ta("t", "a")],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("x")), "c"),
                vec![CorrelationReference::factory_ta("t", "a")],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
        ];

        for test_datum in test_data {
            match AstTranslator::column_reference(
                test_datum.ast_column_reference,
                &test_datum.correlations,
            ) {
                Ok(ffr) => {
                    assert_eq!(ffr, test_datum.expected_result.unwrap())
                }
                Err(e) => {
                    assert_eq!(e.kind(), &test_datum.expected_result.unwrap_err())
                }
            }
        }

        Ok(())
    }
}
