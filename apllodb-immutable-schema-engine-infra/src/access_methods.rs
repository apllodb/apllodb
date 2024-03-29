use std::pin::Pin;

use apllodb_shared_components::ApllodbResult;
use futures::Future;

pub(crate) mod with_db_methods_impl;
pub(crate) mod with_tx_methods_impl;
pub(crate) mod without_db_methods_impl;

type BoxFutRes<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>> + 'static>>;
