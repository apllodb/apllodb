#![allow(missing_docs)]

//! Module to translate [ApllodbAst](apllodb_sql_parser::ApllodbAst) into [apllodb_shared_components](crate)' data structures.

pub mod alias;
pub mod binary_operator;
pub mod column_constraint;
pub mod column_definition;
pub mod column_name;
pub mod column_reference;
pub mod condition;
pub mod data_type;
pub mod database_name;
pub mod expression;
pub mod from_item;
pub mod select;
pub mod select_field;
pub mod table_constraint;
pub mod table_name;
pub mod unary_operator;

/// Holds static translation methods.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AstTranslator;
