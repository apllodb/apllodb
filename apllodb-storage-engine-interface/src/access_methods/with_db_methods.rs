use apllodb_shared_components::{SessionWithDb, SessionWithTx};

use super::FutRes;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithDbMethods {
    fn begin_transaction(self, session: SessionWithDb) -> FutRes<SessionWithTx>;
}
