pub mod constant;

use crate::{
    data_structure::expression::operator::BinaryOperator, ApllodbResult, BooleanExpression,
    ComparisonFunction, CorrelationReference, Expression,
};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn expression_in_select(
        ast_expression: apllodb_ast::Expression,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<Expression> {
        let expression: Expression = match ast_expression {
            apllodb_ast::Expression::ConstantVariant(c) => {
                let sql_value = Self::constant(c)?;
                Expression::ConstantVariant(sql_value)
            }
            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                let ffr = Self::column_reference(ast_colref, correlations)?;
                Expression::FullFieldReferenceVariant(ffr)
            }
            apllodb_ast::Expression::UnaryOperatorVariant(uni_op, expr) => {
                let uni_op = Self::unary_operator(uni_op);
                let expr = Self::expression_in_select(*expr, correlations)?;
                Expression::UnaryOperatorVariant(uni_op, Box::new(expr))
            }
            apllodb_ast::Expression::BinaryOperatorVariant(bin_op, left, right) => {
                let bin_op = Self::binary_operator(bin_op);
                let left = Self::expression_in_select(*left, correlations.clone())?;
                let right = Self::expression_in_select(*right, correlations)?;

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

    pub fn expression_in_non_select(
        ast_expression: apllodb_ast::Expression,
        ast_tables: Vec<apllodb_ast::TableName>,
    ) -> ApllodbResult<Expression> {
        let table_names: Vec<CorrelationReference> = ast_tables
            .into_iter()
            .map(|ast_table_name| {
                let table_name = AstTranslator::table_name(ast_table_name)?;
                Ok(CorrelationReference::TableNameVariant(table_name))
            })
            .collect::<ApllodbResult<_>>()?;
        Self::expression_in_select(ast_expression, &table_names)
    }
}
