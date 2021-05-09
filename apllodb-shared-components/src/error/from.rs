use crate::SqlState;

use super::ApllodbError;
use std::io;

impl From<io::Error> for ApllodbError {
    fn from(ioerr: io::Error) -> Self {
        ApllodbError::new(SqlState::IoError, "IO error", Some(Box::new(ioerr)))
    }
}
