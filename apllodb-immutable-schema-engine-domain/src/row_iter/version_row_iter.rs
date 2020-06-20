use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_engine_interface::Row;
use std::fmt::Debug;

/// Row iterator from a single version.
pub trait VersionRowIter: Iterator<Item = ApllodbResult<Row>> + Debug + Sized {}
