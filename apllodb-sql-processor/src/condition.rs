use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, BooleanExpression, ComparisonFunction,
    Expression, SchemaIndex, SqlValue,
};
use serde::{Deserialize, Serialize};

use crate::Record;

/// Conditional expression. Given a Record, this is evaluated into boolean.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct Condition(Expression);

impl Condition {
    /// # Failures
    ///
    /// - [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - Expression is not a constant.
    ///   - Expression is a constant but it cannot be evaluated as boolean.
    pub(crate) fn eval_as_constant(&self) -> ApllodbResult<bool> {
        match &self.0 {
            Expression::ConstantVariant(sql_value) => sql_value.to_bool(),
            _ => Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                "expression cannot be evaluated as a constant",
                None,
            )),
        }
    }

    /// # Failures
    ///
    /// - [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - Expression cannot be evaluated as BOOLEAN (NULL is OK and evaluated as FALSE).
    pub(crate) fn eval_with_record(&self, record: &Record) -> ApllodbResult<bool> {
        self.0
            .to_sql_value_for_expr_with_index(&|index| {
                record.get_sql_value(index).map(|v| v.clone())
            })
            .and_then(|sql_value| sql_value.to_bool())
    }

    /// # Failures
    ///
    /// - [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch) when:
    ///   - Expression is not a form of single probe.
    pub(crate) fn into_probe(self) -> ApllodbResult<(SchemaIndex, SqlValue)> {
        let err = ApllodbError::new(
            ApllodbErrorKind::DatatypeMismatch,
            "Expression is not a form of single probe (e.g. `c1 = 123`)",
            None,
        );

        match self.0 {
            Expression::BooleanExpressionVariant(b_expr) => match b_expr {
                BooleanExpression::ComparisonFunctionVariant(c_func) => match c_func {
                    ComparisonFunction::EqualVariant { left, right } => match (*left, *right) {
                        (
                            Expression::ConstantVariant(sql_value),
                            Expression::SchemaIndexVariant(schema_index),
                        )
                        | (
                            Expression::SchemaIndexVariant(schema_index),
                            Expression::ConstantVariant(sql_value),
                        ) => Ok((schema_index, sql_value)),

                        _ => Err(err),
                    },
                },
                _ => Err(err),
            },
            _ => Err(err),
        }
    }
}
