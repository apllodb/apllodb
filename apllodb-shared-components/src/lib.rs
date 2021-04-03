#![deny(warnings, missing_debug_implementations, missing_docs)]

//! Data structures shared with multiple crates in the apllodb workspace.

#[macro_use]
extern crate derive_new;

pub(crate) mod ast_translator;
pub(crate) mod data_structure; // TODO remove
pub(crate) mod database;
pub(crate) mod error;
pub(crate) mod session;
pub(crate) mod traits;
pub(crate) mod validation_helper;

pub use crate::{
    ast_translator::AstTranslator,
    data_structure::{
        alias_name::AliasName,
        alter_table_action::AlterTableAction,
        column::{
            column_constraint_kind::ColumnConstraintKind, column_constraints::ColumnConstraints,
            column_data_type::ColumnDataType, column_definition::ColumnDefinition,
            column_name::ColumnName,
        },
        expression::{
            boolean_expression::{
                comparison_function::ComparisonFunction, logical_function::LogicalFunction,
                BooleanExpression,
            },
            operator::UnaryOperator,
            Expression,
        },
        record::{field_index::FieldIndex, Record},
        records::{record_field_ref_schema::RecordFieldRefSchema, Records},
        reference::{
            correlation_reference::{correlation_index::CorrelationIndex, CorrelationReference},
            field_reference::FieldReference,
            full_field_reference::FullFieldReference,
        },
        select::ordering::Ordering,
        table::{
            table_constraint_kind::TableConstraintKind, table_constraints::TableConstraints,
            table_name::TableName,
        },
        value::{
            sql_type::{I64LooseType, NumericComparableType, SqlType, StringComparableLoseType},
            sql_value::{
                nn_sql_value::NnSqlValue, sql_compare_result::SqlCompareResult,
                sql_value_hash_key::SqlValueHashKey, SqlValue,
            },
            sql_values::SqlValues,
        },
    },
    database::database_name::DatabaseName,
    error::{
        kind::ApllodbErrorKind,
        session_error::{ApllodbSessionError, ApllodbSessionResult},
        sqlstate::SqlState,
        ApllodbError, ApllodbResult,
    },
    session::{
        session_id::SessionId, with_db::SessionWithDb, with_tx::SessionWithTx,
        without_db::SessionWithoutDb, Session,
    },
    traits::sql_convertible::SqlConvertible,
};

#[cfg(feature = "test-support")]
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
