use super::{ApllodbError, kind::ApllodbErrorKind};
use std::io;

impl From<io::Error> for ApllodbError {
    fn from(ioerr: io::Error) -> Self {
        ApllodbError::new(ApllodbErrorKind::IoError, "IO error", Some(Box::new(ioerr)))
    }
}
