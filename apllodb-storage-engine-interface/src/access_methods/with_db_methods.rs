use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, Session, SessionId, SessionWithDb,
    SessionWithTx,
};
use futures::FutureExt;

use super::BoxFut;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithDbMethods: Sized + 'static {
    fn begin_transaction(
        self,
        session: SessionWithDb,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = session.get_id().clone();
        async move {
            match self.begin_transaction_core(sid).await {
                Ok(_) => Ok(session.upgrade()),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn begin_transaction_core(self, sid: SessionId) -> BoxFut<ApllodbResult<()>>;
}
