use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, SchemaIndex};
use apllodb_sql_parser::apllodb_ast;

use crate::{
    ast_translator::AstTranslator, attribute::attribute_name::AttributeName,
    correlation::aliased_correlation_name::AliasedCorrelationName, field::field_name::FieldName,
};

impl AstTranslator {
    /// TODO may need Catalog value when:
    /// - ast_column_reference does not have correlation and
    /// - from_item_correlations are more than 1
    /// because this function has to determine which of `from1` or `from2` `field1` is from.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](crate::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `from_item_correlations` is empty.
    /// - [UndefinedColumn](crate::ApllodbErrorKind::UndefinedColumn) when:
    ///   - none of `from_item_correlations` has field named `ast_column_reference.column_name`
    ///   - `ast_column_reference` has a correlation but it is not any of `from_item_correlations`.
    pub fn column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<FieldName> {
        if from_item_correlations.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidColumnReference,
                format!(
                    "no FROM item. cannot detect where `{:?}` field is from",
                    ast_column_reference
                ),
                None,
            ))
        } else if let Some(ast_corr) = ast_column_reference.correlation {
            Self::column_reference_with_corr(
                ast_corr,
                ast_column_reference.column_name,
                from_item_correlations,
            )
        } else {
            Self::column_reference_without_corr(
                ast_column_reference.column_name,
                from_item_correlations,
            )
        }
    }

    /// # Failures
    ///
    /// - [UndefinedColumn](crate::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `ast_correlation` does not match any of `from_item_correlations`.
    fn column_reference_with_corr(
        ast_correlation: apllodb_ast::Correlation,
        ast_column_name: apllodb_ast::ColumnName,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<FieldName> {
        assert!(!from_item_correlations.is_empty());

        let attr = AttributeName::ColumnNameVariant(Self::column_name(ast_column_name)?);
        let index = SchemaIndex::from(format!("{}.{}", ast_correlation.0 .0, attr).as_str());

        // SELECT T.C FROM ...;
        from_item_correlations
            .iter()
            .find_map(|from_item_corr| {
                let field_name = FieldName::new(from_item_corr.clone(), attr.clone());
                if field_name.matches(&index) {
                    Some(field_name)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!(
                        "`{}` does not match any of FROM items: {:?}",
                        index, from_item_correlations
                    ),
                    None,
                )
            })
    }

    fn column_reference_without_corr(
        ast_column_name: apllodb_ast::ColumnName,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<FieldName> {
        assert!(!from_item_correlations.is_empty());
        if from_item_correlations.len() > 1 {
            return Err(ApllodbError::feature_not_supported(format!(
                "needs catalog info to detect which table has the column `{:?}`",
                ast_column_name
            )));
        }

        // SELECT C FROM T (AS a)?;
        // -> C is from T
        let from_item_correlation = from_item_correlations[0].clone();
        let attr = AttributeName::ColumnNameVariant(Self::column_name(ast_column_name)?);
        Ok(FieldName::new(from_item_correlation, attr))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast_translator::AstTranslator,
        correlation::aliased_correlation_name::AliasedCorrelationName,
        field::field_name::FieldName,
    };
    use apllodb_shared_components::ApllodbErrorKind;
    use apllodb_sql_parser::apllodb_ast::{ColumnReference, Correlation};
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ast_column_reference: ColumnReference,
        from_item_correlations: Vec<AliasedCorrelationName>,
        expected_result: Result<FieldName, ApllodbErrorKind>,
    }

    #[test]
    fn test_column_reference() {
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
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(FieldName::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(FieldName::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t1")), "c"),
                vec![AliasedCorrelationName::factory_tn("t2")],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Ok(FieldName::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Ok(FieldName::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("a")), "c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Ok(FieldName::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("x")), "c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
        ];

        for test_datum in test_data {
            match AstTranslator::column_reference(
                test_datum.ast_column_reference,
                &test_datum.from_item_correlations,
            ) {
                Ok(ffr) => {
                    assert_eq!(ffr, test_datum.expected_result.unwrap())
                }
                Err(e) => {
                    assert_eq!(e.kind(), &test_datum.expected_result.unwrap_err())
                }
            }
        }
    }
}
