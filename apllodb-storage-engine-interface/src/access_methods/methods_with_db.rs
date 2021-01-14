use apllodb_shared_components::{ApllodbResult, SessionWithTx};

#[cfg(any(test, feature = "test_support"))]
use mockall::automock;

/// Access methods with open database (without transaction).
#[cfg_attr(any(test, feature = "test_support"), automock)]
pub trait MethodsWithDb {
    /// Begins a transaction.
    fn begin(self) -> ApllodbResult<SessionWithTx>;
}
