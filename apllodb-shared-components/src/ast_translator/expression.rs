pub mod constant;

use crate::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn expression(ast_expression: apllodb_ast::Expression) -> ApllodbResult<Expression> {
        let expression: Expression = match ast_expression {
            apllodb_ast::Expression::ConstantVariant(c) => {
                let sql_value = Self::constant(c)?;
                Expression::ConstantVariant(sql_value)
            }
            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                match ast_colref.correlation {
                    Some(corr) => {
                        // FIXME cannot distinguish whether ast_colref.correlation is table_name / alias.
                        // Needs more info like:
                        // - from_item in SELECT
                        // - table_name in INSERT
                        let ast_table_name = apllodb_ast::TableName(corr.0);
                        let ffr = Self::column_reference_with_table_name(
                            ast_table_name,
                            ast_colref.column_name,
                        )?;
                        Expression::FullFieldReferenceVariant(ffr)
                    }
                    None => {
                        todo!()
                    }
                }
            }
            apllodb_ast::Expression::UnaryOperatorVariant(uni_op, expr) => {
                let uni_op = Self::unary_operator(uni_op);
                let expr = Self::expression(*expr)?;
                Expression::UnaryOperatorVariant(uni_op, Box::new(expr))
            }
        };
        Ok(expression)
    }
}
