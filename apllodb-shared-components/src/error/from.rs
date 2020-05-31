use super::{ApllodbError, ApllodbErrorKind};
use std::io;

impl From<io::Error> for ApllodbError {
    fn from(ioerr: io::Error) -> Self {
        ApllodbError::new(
            ApllodbErrorKind::IoError,
            format!("IO error"),
            Some(Box::new(ioerr)),
        )
    }
}
