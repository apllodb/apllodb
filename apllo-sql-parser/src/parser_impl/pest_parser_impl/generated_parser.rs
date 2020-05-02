#[cfg(test)] // TODO: 今はunusedだからtest限定だが、後で外す
mod pest_result;
#[cfg(test)]
mod tests;

#[cfg(test)] // TODO: 今はunusedだからtest限定だが、後で外す
pub(self) use pest_result::PestResult;

#[cfg(test)] // TODO: 今はunusedだからtest限定だが、後で外す
use pest_derive::Parser;

/// The parser generated from `apllo_sql.pest`.
///
/// pest_derive::Parser macro puts `pub enum Rule` at this level.
#[cfg(test)] // TODO: 今はunusedだからtest限定だが、後で外す
#[derive(Parser)]
#[grammar = "pest_grammar/apllo_sql.pest"]
pub(crate) struct GeneratedParser;
