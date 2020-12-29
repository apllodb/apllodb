use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;

use super::modification_plan::ModificationPlan;

/// Modification (INSERT, UPDATE, and DELETE) executor which inputs a [ModificationPlan](crate::modification_plan::ModificationPlan) and requests modification to storage engine.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(crate) struct ModificationExecutor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> ModificationExecutor<'exe, Engine> {
    pub(crate) fn run(&self, plan: ModificationPlan) -> ApllodbResult<()> {
        // INSERT, UPDATE は、input (元データ) を RecordIterator で取る
        todo!()
    }
}
