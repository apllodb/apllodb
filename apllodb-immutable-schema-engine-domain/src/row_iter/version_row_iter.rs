use crate::ImmutableRow;
use apllodb_shared_components::error::ApllodbResult;
use std::fmt::Debug;

/// Row iterator from a single version.
pub trait VersionRowIter: Iterator<Item = ApllodbResult<ImmutableRow>> + Debug + Sized {}
