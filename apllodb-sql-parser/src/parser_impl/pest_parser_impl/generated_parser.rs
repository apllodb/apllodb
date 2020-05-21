#[cfg(test)]
mod tests;

use pest_derive::Parser;

/// The parser generated from `apllodb_sql.pest`.
///
/// pest_derive::Parser macro puts `pub enum Rule` at this level.
#[derive(Parser)]
#[grammar = "pest_grammar/apllodb_sql.pest"]
pub(super) struct GeneratedParser;
