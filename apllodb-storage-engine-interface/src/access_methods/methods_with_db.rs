use apllodb_shared_components::{ApllodbResult, SessionWithDb, SessionWithTx};

#[cfg(feature = "test-support")]
use mockall::automock;

/// Access methods with open database (without transaction).
#[cfg_attr(feature = "test-support", automock)]
pub trait MethodsWithDb {
    /// Begins a transaction.
    fn begin(self, session: SessionWithDb) -> ApllodbResult<SessionWithTx>;
}
