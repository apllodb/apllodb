use super::Rule;
use pest::error::Error;

pub(super) type PestResult<T> = Result<T, Error<Rule>>;
