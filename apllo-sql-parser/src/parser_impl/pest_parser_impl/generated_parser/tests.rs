mod parse_identifier;
mod parse_keyword;

use super::Rule;
use pest::error::Error;

pub(self) type PestResult<T> = Result<T, Error<Rule>>;
