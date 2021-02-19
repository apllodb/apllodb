pub mod constant;

use crate::{
    data_structure::expression::operator::BinaryOperator, ApllodbResult, BooleanExpression,
    ComparisonFunction, Expression,
};
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
                let ffr = Self::column_reference(ast_colref)?;
                Expression::SelectFieldReferenceVariant(ffr)
            }
            apllodb_ast::Expression::UnaryOperatorVariant(uni_op, expr) => {
                let uni_op = Self::unary_operator(uni_op);
                let expr = Self::expression(*expr)?;
                Expression::UnaryOperatorVariant(uni_op, Box::new(expr))
            }
            apllodb_ast::Expression::BinaryOperatorVariant(bin_op, left, right) => {
                let bin_op = Self::binary_operator(bin_op);
                let left = Self::expression(*left)?;
                let right = Self::expression(*right)?;

                match bin_op {
                    BinaryOperator::Equal => Expression::BooleanExpressionVariant(
                        BooleanExpression::ComparisonFunctionVariant(
                            ComparisonFunction::EqualVariant {
                                left: Box::new(left),
                                right: Box::new(right),
                            },
                        ),
                    ),
                }
            }
        };
        Ok(expression)
    }
}
