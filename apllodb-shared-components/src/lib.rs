#![deny(warnings, missing_debug_implementations, missing_docs)]

//! Data structures shared with multiple crates in the apllodb workspace.

#[macro_use]
extern crate derive_new;

pub(crate) mod database;
pub(crate) mod error;
pub(crate) mod expression;
pub(crate) mod schema;
pub(crate) mod session;
pub(crate) mod validation_helper;
pub(crate) mod value;

pub use crate::{
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
    schema::{r_pos::RPos, schema_index::SchemaIndex, schema_name::SchemaName, Schema},
    session::{
        session_id::SessionId, with_db::SessionWithDb, with_tx::SessionWithTx,
        without_db::SessionWithoutDb, Session,
    },
    validation_helper::{
        collection::{find_dup, find_dup_slow},
        short_name::ShortName,
    },
    value::{
        sql_convertible::SqlConvertible,
        sql_type::{I64LooseType, NumericComparableType, SqlType, StringComparableLoseType},
        sql_value::{
            nn_sql_value::NnSqlValue, sql_compare_result::SqlCompareResult,
            sql_value_hash_key::SqlValueHashKey, SqlValue,
        },
    },
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
