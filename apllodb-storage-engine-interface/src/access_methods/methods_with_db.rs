use std::fmt::Debug;

use apllodb_shared_components::{ApllodbResult, SessionWithTx};

/// Access methods with open database (without transaction).
pub trait MethodsWithDb: Debug {
    /// Begins a transaction.
    fn begin(self) -> ApllodbResult<SessionWithTx>;
}
