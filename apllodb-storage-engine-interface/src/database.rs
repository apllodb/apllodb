use apllodb_shared_components::DatabaseName;

/// Database interface.
pub trait Database {
    /// Ref to [DatabaseName](crate::DatabaseName).
    fn name(&self) -> &DatabaseName;
}
