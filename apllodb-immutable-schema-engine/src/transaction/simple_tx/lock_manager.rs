use super::SimpleTxId;
use apllodb_shared_components::data_structure::TableName;
use std::collections::HashMap;

/// Token that both readers and writers to a table must acquired before reading/writing.
///
/// TODO make it !Send.
#[derive(Hash, Debug)]
pub(super) struct TableRwToken(TableName);

/// Provides reentrant try-lock to table (table lock).
///
/// Fields are not wrapped in latch, assuming LockManager's singleton instance is accessed via latch.
#[derive(Debug)]
pub(crate) struct LockManager {
    lock_table: HashMap<TableName, SimpleTxId>,
}

impl LockManager {
    /// Create an instance, which is intended to be used as singleton.
    pub(crate) fn new() -> Self {
        Self {
            lock_table: HashMap::new(),
        }
    }

    /// Reentrant try_lock to a table.
    pub(super) fn reentrant_try_lock(
        &mut self,
        table_name: &TableName,
        tx_id: SimpleTxId,
    ) -> Option<TableRwToken> {
        match self.lock_table.entry(table_name.clone()) {
            std::collections::hash_map::Entry::Occupied(e) => {
                if *e.get() == tx_id {
                    Some(TableRwToken(table_name.clone()))
                } else {
                    None
                }
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(tx_id);
                Some(TableRwToken(table_name.clone()))
            }
        }
    }

    pub(super) fn unlock_all(&mut self, tx_id: SimpleTxId) {
        let locked_tables: Vec<TableName> = self
            .lock_table
            .iter()
            .filter(|(_, v)| **v == tx_id)
            .map(|(k, _)| k)
            .cloned()
            .collect();

        for t in locked_tables {
            self.lock_table.remove(&t);
        }
    }
}
