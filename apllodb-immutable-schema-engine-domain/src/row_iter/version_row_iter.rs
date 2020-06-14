use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_engine_interface::Row;

/// Row iterator from a single version.
pub trait VersionRowIter: Iterator<Item = ApllodbResult<Row>> {}
