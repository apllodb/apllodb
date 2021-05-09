use apllodb_shared_components::{ApllodbError, ApllodbResult, Expression};
use apllodb_storage_engine_interface::{RowSelectionQuery, SingleTableCondition, TableName};
use serde::{Deserialize, Serialize};

use crate::Record;

/// Conditional expression. Given a Record, this is evaluated into boolean.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct Condition(Expression);

impl Condition {
    /// # Failures
    ///
    /// - [DataExceptionIllegalOperation](apllodb-shared-components::SqlState::DataExceptionIllegalOperation) when:
    ///   - Expression is not a constant.
    ///   - Expression is a constant but it cannot be evaluated as boolean.
    pub(crate) fn eval_as_boolean_constant(&self) -> ApllodbResult<bool> {
        match &self.0 {
            Expression::ConstantVariant(sql_value) => sql_value.to_bool(),
            _ => Err(ApllodbError::data_exception_illegal_operation(
                "expression cannot be evaluated as a constant",
            )),
        }
    }

    /// # Failures
    ///
    /// - [DataExceptionIllegalOperation](apllodb-shared-components::SqlState::DataExceptionIllegalOperation) when:
    ///   - Expression cannot be evaluated as BOOLEAN (NULL is OK and evaluated as FALSE).
    pub(crate) fn eval_with_record(&self, record: &Record) -> ApllodbResult<bool> {
        self.0
            .to_sql_value_for_expr_with_index(&|index| {
                record.get_sql_value(index).map(|v| v.clone())
            })
            .and_then(|sql_value| sql_value.to_bool())
    }

    /// # Panics
    ///
    /// Expression contain two or more tables in SchemaIndexVariant's.
    pub(crate) fn into_row_selection_query(self, table_name: TableName) -> RowSelectionQuery {
        let single_table_condition = SingleTableCondition::new(table_name, self.0);
        RowSelectionQuery::Condition(single_table_condition)
    }
}
