use apllodb_shared_components::{SessionWithDb, SessionWithTx};

use super::FutRes;

pub trait WithDbMethods {
    fn begin_transaction(self, session: SessionWithDb) -> FutRes<SessionWithTx>;
}
