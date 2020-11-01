use crate::row::immutable_row::ImmutableRow;
use apllodb_shared_components::error::ApllodbResult;
use std::fmt::Debug;

/// Row iterator from a single version.
pub trait VersionRowIterator: Iterator<Item = ApllodbResult<ImmutableRow>> + Debug + Sized {}
