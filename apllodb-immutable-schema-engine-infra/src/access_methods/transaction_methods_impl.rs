use apllodb_shared_components::{ApllodbResult, SessionWithDb, TransactionId};
use apllodb_storage_engine_interface::TransactionMethods;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct TransactionMethodsImpl;

impl TransactionMethods for TransactionMethodsImpl {
    fn begin_core(&self, session: &mut SessionWithDb) -> ApllodbResult<TransactionId> {
        todo!()
    }

    fn commit_core(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        todo!()
    }

    fn abort_core(&self, session: &mut SessionWithDb) -> ApllodbResult<()> {
        todo!()
    }
}
