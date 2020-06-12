use apllodb_shared_components::data_structure::DatabaseName;

/// Database context interface.
///
/// An instance of this trait implementation is typically shared among threads (transactions).
/// Fields to be updated should have interior mutability (like `std::sync::Mutex`).
pub trait DbCtxLike {
    /// Ref to [DatabaseName](foobar.html).
    fn name(&self) -> &DatabaseName;
}
