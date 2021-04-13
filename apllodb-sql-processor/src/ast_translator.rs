#![allow(missing_docs)]

//! Module to translate [ApllodbAst](apllodb_sql_parser::ApllodbAst) into [apllodb_shared_components](crate)' data structures.

pub(crate) mod alias;
pub(crate) mod alter_table_action;
pub(crate) mod column_constraint;
pub(crate) mod column_definition;
pub(crate) mod column_name;
pub(crate) mod condition;
pub(crate) mod data_type;
pub(crate) mod database_name;
pub(crate) mod expression;
pub(crate) mod select;
pub(crate) mod table_constraint;
pub(crate) mod table_name;

/// Holds static translation methods.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct AstTranslator;
