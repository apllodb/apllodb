use crate::{AliasName, ApllodbResult, CorrelationReference, FullFieldReference};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// # Failures
    ///
    /// errors from [AstTranslator::column_reference()](crate::AstTranslator::column_reference).
    pub fn select_field_column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
        ast_field_alias: Option<apllodb_ast::Alias>,
        correlations: Vec<CorrelationReference>,
    ) -> ApllodbResult<FullFieldReference> {
        let mut ffr = Self::column_reference(ast_column_reference, correlations)?;
        if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(field_alias))) = ast_field_alias {
            ffr.set_field_alias(AliasName::new(field_alias)?);
        }
        Ok(ffr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApllodbErrorKind, ApllodbResult, AstTranslator, FullFieldReference};
    use apllodb_sql_parser::apllodb_ast::{Alias, ColumnReference, Correlation, FromItem};
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ast_column_reference: ColumnReference,
        ast_field_alias: Option<Alias>,
        ast_from_items: Vec<FromItem>,
        expected_result: Result<FullFieldReference, ApllodbErrorKind>,
    }

    #[test]
    fn test_select_field_column_reference() -> ApllodbResult<()> {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                None,
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                None,
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("corr_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("corr_alias")), "c"),
                None,
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("corr_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                Some(Alias::factory("field_alias")),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c").with_field_alias("field_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                Some(Alias::factory("field_alias")),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c").with_field_alias("field_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                Some(Alias::factory("field_alias")),
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c")
                    .with_corr_alias("corr_alias")
                    .with_field_alias("field_alias")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("corr_alias")), "c"),
                Some(Alias::factory("field_alias")),
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c")
                    .with_corr_alias("corr_alias")
                    .with_field_alias("field_alias")),
            ),
        ];

        for test_datum in test_data {
            match AstTranslator::select_field_column_reference(
                test_datum.ast_column_reference,
                test_datum.ast_field_alias,
                test_datum.ast_from_items,
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
