use super::Storage;
use apllodb_shared_components::error::ApllodbResult;

/// Sync-All storage.
///
/// It loads all the tables, versions, and records from disk.
/// It modifies them in memory.
/// It flushes all of them from memory into disk.
///
/// Really simple to implement and really poor in performance.
#[derive(Hash, Debug)]
pub(crate) struct SyncAllStorage;

impl Storage for SyncAllStorage {
    fn make_durable(&self) -> ApllodbResult<()> {
        todo!()
    }
}
