pub(crate) mod binary_operator;
pub(crate) mod constant;
pub(crate) mod unary_operator;

use apllodb_shared_components::{
    ApllodbResult, BinaryOperator, BooleanExpression, ComparisonFunction, Expression, SchemaIndex,
};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::TableName;

use crate::{
    ast_translator::AstTranslator,
    correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_name::CorrelationName,
    },
};

impl AstTranslator {
    pub fn expression_in_select(
        ast_expression: apllodb_ast::Expression,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<Expression> {
        let expression: Expression = match ast_expression {
            apllodb_ast::Expression::ConstantVariant(c) => {
                let sql_value = Self::constant(c)?;
                Expression::ConstantVariant(sql_value)
            }
            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                let field_name = Self::column_reference(ast_colref, from_item_correlations)?;
                Expression::SchemaIndexVariant(SchemaIndex::from(&field_name))
            }
            apllodb_ast::Expression::UnaryOperatorVariant(uni_op, expr) => {
                let uni_op = Self::unary_operator(uni_op);
                let expr = Self::expression_in_select(*expr, from_item_correlations)?;
                Expression::UnaryOperatorVariant(uni_op, Box::new(expr))
            }
            apllodb_ast::Expression::BinaryOperatorVariant(bin_op, left, right) => {
                let bin_op = Self::binary_operator(bin_op);
                let left = Self::expression_in_select(*left, from_item_correlations)?;
                let right = Self::expression_in_select(*right, from_item_correlations)?;

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
        table_names: Vec<TableName>,
    ) -> ApllodbResult<Expression> {
        let corrs: Vec<AliasedCorrelationName> = table_names
            .into_iter()
            .map(|table_name| {
                let corr_name = CorrelationName::TableNameVariant(table_name);
                AliasedCorrelationName::new(corr_name, None)
            })
            .collect();
        Self::expression_in_select(ast_expression, &corrs)
    }
}
