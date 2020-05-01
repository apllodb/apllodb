mod pest_result;
#[cfg(test)]
mod tests;

pub(in crate::pest_parser) use pest_result::PestResult;

use pest_derive::Parser;

/// The parser generated from `apllo_sql.pest`.
///
/// pest_derive::Parser macro puts `pub enum Rule` at this level.
#[derive(Parser)]
#[grammar = "pest_grammar/apllo_sql.pest"]
pub(crate) struct PestParser;
