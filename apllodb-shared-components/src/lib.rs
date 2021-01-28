#![deny(warnings, missing_debug_implementations, missing_docs)]

//! Data structures shared with multiple crates in the apllodb workspace.

#[macro_use]
extern crate derive_new;

pub(crate) mod data_structure;
pub(crate) mod error;
pub(crate) mod traits;

pub use crate::{
    data_structure::{
        alter_table_action::AlterTableAction,
        column::{
            column_constraint_kind::ColumnConstraintKind, column_constraints::ColumnConstraints,
            column_data_type::ColumnDataType, column_definition::ColumnDefinition,
            column_name::ColumnName, column_reference::ColumnReference, column_value::ColumnValue,
        },
        database::database_name::DatabaseName,
        expression::{
            boolean_expression::{
                comparison_function::ComparisonFunction, logical_function::LogicalFunction,
                BooleanExpression,
            },
            Expression,
        },
        record::{field_index::FieldIndex, Record},
        record_iterator::RecordIterator,
        session::{
            session_id::SessionId, with_db::SessionWithDb, with_tx::SessionWithTx,
            without_db::SessionWithoutDb, Session,
        },
        table::{
            table_constraint_kind::TableConstraintKind, table_constraints::TableConstraints,
            table_name::TableName,
        },
        value::{
            sql_type::{I64LooseType, NumericComparableType, SqlType, StringComparableLoseType},
            sql_value::{
                nn_sql_value::NNSqlValue, sql_compare_result::SqlCompareResult,
                sql_value_hash_key::SqlValueHashKey, SqlValue,
            },
        },
    },
    error::{
        kind::ApllodbErrorKind,
        session_error::{ApllodbSessionError, ApllodbSessionResult},
        sqlstate::SqlState,
        ApllodbError, ApllodbResult,
    },
    traits::sql_convertible::SqlConvertible,
};

#[cfg(feature = "test-support")]
#[allow(missing_docs)]
pub mod test_support;

#[cfg(test)]
mod tests {
    use apllodb_test_support::setup::setup_test_logger;
    use ctor::ctor;

    #[cfg_attr(test, ctor)]
    fn test_setup() {
        setup_test_logger();
    }
}
