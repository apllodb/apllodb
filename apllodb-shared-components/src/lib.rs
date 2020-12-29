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
        data_type::{data_type_kind::DataTypeKind, DataType},
        database::database_name::DatabaseName,
        expression::{
            boolean_expression::{
                comparison_function::ComparisonFunction, logical_function::LogicalFunction,
                BooleanExpression,
            },
            constant::{
                character_constant::{CharacterConstant, TextConstant},
                numeric_constant::{IntegerConstant, NumericConstant},
                Constant,
            },
            Expression,
        },
        record::{field_index::FieldIndex, Record},
        record_iterator::RecordIterator,
        table::{
            table_constraint_kind::TableConstraintKind, table_constraints::TableConstraints,
            table_name::TableName,
        },
        value::sql_value::{
            sql_compare_result::SqlCompareResult, sql_value_hash_key::SqlValueHashKey, SqlValue,
        },
    },
    error::{kind::ApllodbErrorKind, sqlstate::SqlState, ApllodbError, ApllodbResult},
    traits::{database::Database, sql_convertible::SqlConvertible},
};

#[cfg(test)]
pub mod test_support;
