use apllodb_shared_components::{SessionWithDb, SessionWithTx};
use mockall::automock;

use super::FutRes;

#[cfg_attr(feature = "test-support", automock)]
pub trait WithDbMethods {
    fn begin_transaction(self, session: SessionWithDb) -> FutRes<SessionWithTx>;
}
