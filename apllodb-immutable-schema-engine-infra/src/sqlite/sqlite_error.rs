use apllodb_shared_components::{ApllodbError, ApllodbErrorKind};

pub(in crate::sqlite) fn map_sqlite_err<S: Into<String>>(
    e: rusqlite::Error,
    desc: S,
) -> ApllodbError {
    ApllodbError::new(ApllodbErrorKind::IoError, desc, Some(Box::new(e)))
}
