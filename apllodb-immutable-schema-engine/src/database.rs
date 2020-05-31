use crate::{latch::Latch, transaction::LockManager};
use apllodb_shared_components::data_structure::DatabaseName;
use apllodb_storage_manager_interface::DbCtxLike;
use std::sync::Arc;

/// Database context.
#[derive(Debug)]
pub struct Database {
    name: DatabaseName,

    // Singleton (1 per database) instance of LockManager.
    // Arc since many Tx instances share the same LockManager instance.
    // Latch since a Tx instance should exclusively borrow mutable reference to LockManager at a time to lock/unlock.
    pub(crate) lock_manager: Arc<Latch<LockManager>>,
}

impl DbCtxLike for Database {
    fn name(&self) -> &DatabaseName {
        &self.name
    }
}

impl Database {
    /// Constructor.
    pub fn new(db_name: DatabaseName) -> Self {
        let lock_manager = Arc::new(Latch::new(LockManager::new()));
        Self {
            name: db_name,
            lock_manager,
        }
    }
}
