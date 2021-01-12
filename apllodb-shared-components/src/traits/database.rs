use crate::{ApllodbResult, DatabaseName};
use std::fmt::Debug;

/// Database interface.
pub trait Database: Debug + Sized {
    /// Start using a database.
    fn use_database(name: DatabaseName) -> ApllodbResult<Self>;

    /// Ref to [DatabaseName](crate::DatabaseName).
    fn name(&self) -> &DatabaseName;
}

