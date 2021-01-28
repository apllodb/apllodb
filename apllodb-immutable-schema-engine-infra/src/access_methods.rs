use std::pin::Pin;

use apllodb_shared_components::ApllodbSessionResult;
use futures::Future;

pub(crate) mod with_db_methods_impl;
pub(crate) mod with_tx_methods_impl;
pub(crate) mod without_db_methods_impl;

type FutRes<S> = Pin<Box<dyn Future<Output = ApllodbSessionResult<S>> + 'static>>;
