use std::pin::Pin;

use apllodb_shared_components::ApllodbResult;
use futures::Future;

pub type FutRes<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>> + 'static>>;

pub(crate) mod with_db_methods;
pub(crate) mod with_tx_methods;
pub(crate) mod without_db_methods;
