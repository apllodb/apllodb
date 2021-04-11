use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast::{self};

use crate::{
    ast_translator::AstTranslator,
    correlation::aliased_correlation_name::AliasedCorrelationName,
    field::{aliased_field_name::AliasedFieldName, field_alias::FieldAlias},
};

impl AstTranslator {
    /// # Failures
    ///
    /// errors from [AstTranslator::column_reference()](crate::AstTranslator::column_reference).
    pub fn select_field_column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
        ast_field_alias: Option<apllodb_ast::Alias>,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<AliasedFieldName> {
        let field_name = Self::column_reference(ast_column_reference, from_item_correlations)?;
        let aliased_field_name =
            if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(field_alias))) = ast_field_alias
            {
                let field_alias = FieldAlias::new(field_alias)?;
                AliasedFieldName::new(field_name, Some(field_alias))
            } else {
                AliasedFieldName::new(field_name, None)
            };
        Ok(aliased_field_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast_translator::AstTranslator,
        correlation::aliased_correlation_name::AliasedCorrelationName,
        field::aliased_field_name::AliasedFieldName,
    };
    use apllodb_shared_components::ApllodbErrorKind;
    use apllodb_sql_parser::apllodb_ast::{Alias, ColumnReference, Correlation};
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ast_column_reference: ColumnReference,
        ast_field_alias: Option<Alias>,
        from_item_correlations: Vec<AliasedCorrelationName>,
        expected_result: Result<AliasedFieldName, ApllodbErrorKind>,
    }

    #[test]
    fn test_select_field_column_reference() {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                None,
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(AliasedFieldName::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                None,
                vec![AliasedCorrelationName::factory_tn("t").with_alias("corr_alias")],
                Ok(AliasedFieldName::factory("t", "c").with_corr_alias("corr_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("corr_alias")), "c"),
                None,
                vec![AliasedCorrelationName::factory_tn("t").with_alias("corr_alias")],
                Ok(AliasedFieldName::factory("t", "c").with_corr_alias("corr_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                Some(Alias::factory("field_alias")),
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(AliasedFieldName::factory("t", "c").with_field_alias("field_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                Some(Alias::factory("field_alias")),
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(AliasedFieldName::factory("t", "c").with_field_alias("field_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                Some(Alias::factory("field_alias")),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("corr_alias")],
                Ok(AliasedFieldName::factory("t", "c")
                    .with_corr_alias("corr_alias")
                    .with_field_alias("field_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("corr_alias")), "c"),
                Some(Alias::factory("field_alias")),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("corr_alias")],
                Ok(AliasedFieldName::factory("t", "c")
                    .with_corr_alias("corr_alias")
                    .with_field_alias("field_alias")),
            ),
        ];

        for test_datum in test_data {
            match AstTranslator::select_field_column_reference(
                test_datum.ast_column_reference,
                test_datum.ast_field_alias,
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
