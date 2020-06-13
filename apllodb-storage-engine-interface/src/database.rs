use apllodb_shared_components::data_structure::DatabaseName;

/// Database interface.
pub trait Database {
    /// Ref to [DatabaseName](foobar.html).
    fn name(&self) -> &DatabaseName;
}
