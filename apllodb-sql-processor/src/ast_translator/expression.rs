pub(crate) mod constant;

use apllodb_shared_components::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn expression(ast_expression: apllodb_ast::Expression) -> ApllodbResult<Expression> {
        let expression: Expression = match ast_expression {
            apllodb_ast::Expression::ConstantVariant(c) => {
                let sql_value = Self::constant(c)?;
                Expression::ConstantVariant(sql_value)
            }
            apllodb_ast::Expression::ColumnReferenceVariant(colref) => {
                let colref = Self::column_reference(colref)?;
                Expression::ColumnNameVariant(colref.as_column_name().clone())
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
