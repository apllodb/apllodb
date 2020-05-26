mod sync_all;

use apllodb_shared_components::error::ApllodbResult;
pub(crate) use sync_all::SyncAllStorage;

/// A storage deals with:
///
/// - Manipulation of objects (tables, versions, and records).
/// - Make objects durable.
/// - Redo/Undo log for fault recovery.
///
/// and does NOT deal with:
///
/// - Locking/Unlocking objects (transaction manager should handle them).
///
/// Common implementation targets are:
///
/// - Page-based buffer pool with ARIES method.
///   - Appropriate for disk-oriented system.
/// - One specialized in in-memory transaction. Cicada (https://hyeontaek.com/papers/cicada-sigmod2017.pdf), for example.
pub(crate) trait Storage {
    fn make_durable(&self) -> ApllodbResult<()>;
}
