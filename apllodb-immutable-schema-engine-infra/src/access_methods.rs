use std::pin::Pin;

use apllodb_shared_components::ApllodbResult;
use futures::Future;

pub(crate) mod without_db_methods_impl;

type FutRes<S: Sync + Send + 'static> = Pin<Box<dyn Future<Output = ApllodbResult<S>>>>;
