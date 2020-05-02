mod parse_identifier;
mod parse_key_word;

use super::Rule;
use pest::error::Error;

pub(self) type PestResult<T> = Result<T, Error<Rule>>;
