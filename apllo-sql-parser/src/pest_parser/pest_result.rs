use super::Rule;
use pest::error::Error;

pub(in crate::pest_parser) type PestResult<T> = Result<T, Error<Rule>>;
