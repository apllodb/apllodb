use crate::{latch::Latch, transaction::LockManager};
use apllodb_storage_manager_interface::DbCtxLike;
use std::sync::Arc;

/// Database context.
#[derive(Debug)]
pub struct Database {
    pub(crate) lock_manager: Arc<Latch<LockManager>>,
}

impl DbCtxLike for Database {}

impl Database {
    /// Constructor.
    pub fn new() -> Self {
        let lock_manager = Arc::new(Latch::new(LockManager::new()));
        Self { lock_manager }
    }
}
