//! Module to translate [ApllodbAst](apllodb_sql_parser::ApllodbAst) into [apllodb_shared_components](apllodb_shared_components)' data structures.

pub(crate) mod column_constraint;
pub(crate) mod column_definition;
pub(crate) mod column_name;
pub(crate) mod constant;
pub(crate) mod data_type;
pub(crate) mod table_name;

#[allow(dead_code)]
/// Holds static translation methods.
pub(crate) struct AstTranslator;
