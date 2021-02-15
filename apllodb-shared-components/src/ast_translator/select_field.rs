use crate::{AliasName, ApllodbResult, FullFieldReference};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// # Failures
    ///
    /// errors from [AstTranslator::column_reference()](crate::AstTranslator::column_reference).
    pub fn select_field_column_reference(
        ast_select_field: apllodb_ast::SelectField,
        ast_from_items: Vec<apllodb_ast::FromItem>,
    ) -> ApllodbResult<FullFieldReference> {
        let ast_expression = ast_select_field.expression;
        let ast_field_alias = ast_select_field.alias;

        match ast_expression {
            apllodb_ast::Expression::ConstantVariant(_)
            | apllodb_ast::Expression::UnaryOperatorVariant(_, _)
            | apllodb_ast::Expression::BinaryOperatorVariant(_, _, _) => {
                panic!("select_field_column_reference() is only for ColumnReferenceVariant")
            }

            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                let mut ffr = Self::column_reference(ast_colref, ast_from_items)?;
                if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(field_alias))) =
                    ast_field_alias
                {
                    ffr.set_field_alias(AliasName::new(field_alias)?);
                }
                Ok(ffr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApllodbErrorKind, ApllodbResult, AstTranslator, FullFieldReference};
    use apllodb_sql_parser::apllodb_ast::{
        ColumnReference, Correlation, Expression, FromItem, SelectField,
    };
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ast_select_field: SelectField,
        ast_from_items: Vec<FromItem>,
        expected_result: Result<FullFieldReference, ApllodbErrorKind>,
    }

    #[test]
    fn test_select_field_column_reference() -> ApllodbResult<()> {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "c")),
                    None,
                ),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c")),
            ),
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(
                        Some(Correlation::factory("t")),
                        "c",
                    )),
                    None,
                ),
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("corr_alias")),
            ),
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(
                        Some(Correlation::factory("corr_alias")),
                        "c",
                    )),
                    None,
                ),
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("corr_alias")),
            ),
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(None, "c")),
                    Some("field_alias"),
                ),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c").with_field_alias("field_alias")),
            ),
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(
                        Some(Correlation::factory("t")),
                        "c",
                    )),
                    Some("field_alias"),
                ),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c").with_field_alias("field_alias")),
            ),
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(
                        Some(Correlation::factory("t")),
                        "c",
                    )),
                    Some("field_alias"),
                ),
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c")
                    .with_corr_alias("corr_alias")
                    .with_field_alias("field_alias")),
            ),
            TestDatum::new(
                SelectField::factory(
                    Expression::factory_colref(ColumnReference::factory(
                        Some(Correlation::factory("corr_alias")),
                        "c",
                    )),
                    Some("field_alias"),
                ),
                vec![FromItem::factory_tn("t", Some("corr_alias"))],
                Ok(FullFieldReference::factory("t", "c")
                    .with_corr_alias("corr_alias")
                    .with_field_alias("field_alias")),
            ),
        ];

        for test_datum in test_data {
            match AstTranslator::select_field_column_reference(
                test_datum.ast_select_field,
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
