use crate::{AliasName, ApllodbResult, UnresolvedFieldReference};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// # Failures
    ///
    /// errors from [AstTranslator::column_reference()](crate::AstTranslator::column_reference).
    pub fn select_field_column_reference(
        ast_select_field: apllodb_ast::SelectField,
    ) -> ApllodbResult<UnresolvedFieldReference> {
        let ast_expression = ast_select_field.expression;
        let ast_field_alias = ast_select_field.alias;

        match ast_expression {
            apllodb_ast::Expression::ConstantVariant(_)
            | apllodb_ast::Expression::UnaryOperatorVariant(_, _)
            | apllodb_ast::Expression::BinaryOperatorVariant(_, _, _) => {
                panic!("select_field_column_reference() is only for ColumnReferenceVariant")
            }

            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                let mut ufr = Self::column_reference(ast_colref)?;
                if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(field_alias))) =
                    ast_field_alias
                {
                    ufr.set_field_alias(AliasName::new(field_alias)?);
                }
                Ok(ufr)
            }
        }
    }
}
