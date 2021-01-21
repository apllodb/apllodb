use std::pin::Pin;

use apllodb_shared_components::ApllodbResult;
use futures::Future;

mod database;

type BoxFutResult<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>>>>;
