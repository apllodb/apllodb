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
            apllodb_ast::Expression::ColumnReferenceVariant(colref) => match colref.correlation {
                Some(corr) => match corr {
                    apllodb_ast::Correlation::TableNameVariant(tn) => {
                        let ffr = Self::column_reference_with_table_name(tn, colref.column_name)?;
                        Expression::FullFieldReferenceVariant(ffr)
                    }
                    apllodb_ast::Correlation::AliasVariant(_) => {
                        todo!()
                    }
                },
                None => {
                    todo!()
                }
            },
            apllodb_ast::Expression::UnaryOperatorVariant(uni_op, expr) => {
                let uni_op = Self::unary_operator(uni_op);
                let expr = Self::expression(*expr)?;
                Expression::UnaryOperatorVariant(uni_op, Box::new(expr))
            }
        };
        Ok(expression)
    }
}
