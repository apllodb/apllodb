//! Module to translate [ApllodbAst](apllodb_sql_parser::ApllodbAst) into [apllodb_shared_components](apllodb_shared_components)' data structures.

pub(crate) mod constant;
pub(crate) mod table_name;
pub(crate) mod column_name;

#[allow(dead_code)]
/// Holds static translation methods.
pub(crate) struct AstTranslator;
