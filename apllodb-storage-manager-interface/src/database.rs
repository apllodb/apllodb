/// Database context interface.
///
/// An instance of this trait implementation is typically shared among threads (transactions).
/// Fields to be updated should have interior mutability (like `std::sync::Mutex`).
pub trait DbCtxLike {}
