use crate::data_structure::database::database_name::DatabaseName;

/// Database interface.
pub trait Database {
    /// Ref to [DatabaseName](crate::DatabaseName).
    fn name(&self) -> &DatabaseName;
}
