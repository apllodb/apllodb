use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, Expression};
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
}
