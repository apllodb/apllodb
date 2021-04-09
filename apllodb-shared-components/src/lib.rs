#![deny(
    // TODO comment-in
    // warnings,
    missing_debug_implementations, 
    missing_docs
)]

//! Data structures shared with multiple crates in the apllodb workspace.

#[macro_use]
extern crate derive_new;

pub(crate) mod schema;
pub(crate) mod attribute; // TODO move to sql-processor
pub(crate) mod correlation; // TODO move to sql-processor
pub(crate) mod data_structure; // TODO remove
pub(crate) mod database;
pub(crate) mod error;
pub(crate) mod expression;
pub(crate) mod field; // TODO move to sql-processor
pub(crate) mod table_column_name; // TODO move to storage-engine
pub(crate) mod session;
pub(crate) mod validation_helper;
pub(crate) mod value;

pub use crate::{
    schema::{Schema, schema_name::SchemaName, schema_index::SchemaIndex, r_pos::RPos},
    table_column_name::TableColumnName,
    data_structure::{
        alias_name::AliasName,
        alter_table_action::AlterTableAction,
        column::{
            column_constraint_kind::ColumnConstraintKind, column_constraints::ColumnConstraints,
            column_data_type::ColumnDataType, column_definition::ColumnDefinition,
            column_name::ColumnName,
        },
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
    },
    database::database_name::DatabaseName,
    error::{
        kind::ApllodbErrorKind,
        session_error::{ApllodbSessionError, ApllodbSessionResult},
        sqlstate::SqlState,
        ApllodbError, ApllodbResult,
    },
    expression::{
        boolean_expression::{
            comparison_function::ComparisonFunction, logical_function::LogicalFunction,
            BooleanExpression,
        },
        operator::{BinaryOperator, UnaryOperator},
        Expression,
    },
    session::{
        session_id::SessionId, with_db::SessionWithDb, with_tx::SessionWithTx,
        without_db::SessionWithoutDb, Session,
    },
    value::{
        sql_convertible::SqlConvertible,
        sql_type::{I64LooseType, NumericComparableType, SqlType, StringComparableLoseType},
        sql_value::{
            nn_sql_value::NnSqlValue, sql_compare_result::SqlCompareResult,
            sql_value_hash_key::SqlValueHashKey, SqlValue,
        },
        sql_values::SqlValues,
    },
};

pub(crate) use crate::{
    attribute::attribute_name::AttributeName,
    correlation::{correlation_alias::CorrelationAlias, correlation_name::CorrelationName},
    field::{aliased_field_name::AliasedFieldName, field_alias::FieldAlias, field_name::FieldName},
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
