use apllodb_shared_components::{
    BooleanExpression, ComparisonFunction, Expression, LogicalFunction,
};
use serde::{Deserialize, Serialize};

use crate::TableName;

/// WHERE condition for a single table.
/// Has Expression inside whose SchemaIndexVariant's , if any, refer only to the specified table.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SingleTableCondition {
    table_name: TableName,
    expression: Expression,
}

impl SingleTableCondition {
    /// # Panics
    ///
    /// Expression contain two or more tables in SchemaIndexVariant's.
    pub fn new(table_name: TableName, expression: Expression) -> Self {
        fn validate_unknown_table_in_defendants(tbl: &TableName, expr: &Expression) {
            match expr {
                Expression::SchemaIndexVariant(index) => {
                    if let Some(prefix) = index.prefix() {
                        assert_eq!(prefix, tbl.as_str(), "expression contains reference to table `{}`, while only {:?} is allowed for this SingleTableCondition", prefix, tbl);
                    }
                }
                Expression::ConstantVariant(_) => {}
                Expression::UnaryOperatorVariant(_, un_expr) => {
                    validate_unknown_table_in_defendants(tbl, un_expr);
                }
                Expression::BooleanExpressionVariant(bin_expr) => match bin_expr {
                    BooleanExpression::LogicalFunctionVariant(lf) => match lf {
                        LogicalFunction::AndVariant { left, right } => {
                            validate_unknown_table_in_defendants(
                                tbl,
                                &Expression::BooleanExpressionVariant(*left.clone()),
                            );
                            validate_unknown_table_in_defendants(
                                tbl,
                                &Expression::BooleanExpressionVariant(*right.clone()),
                            );
                        }
                    },
                    BooleanExpression::ComparisonFunctionVariant(cf) => match cf {
                        ComparisonFunction::EqualVariant { left, right } => {
                            validate_unknown_table_in_defendants(tbl, left.as_ref());
                            validate_unknown_table_in_defendants(tbl, right.as_ref());
                        }
                    },
                },
            }
        }

        validate_unknown_table_in_defendants(&table_name, &expression);

        Self {
            table_name,
            expression,
        }
    }

    /// Table name
    pub fn as_table_name(&self) -> &TableName {
        &self.table_name
    }

    /// Expression
    pub fn as_expression(&self) -> &Expression {
        &self.expression
    }
}
