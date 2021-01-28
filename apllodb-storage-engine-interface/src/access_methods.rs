use std::pin::Pin;

use futures::Future;

pub type BoxFut<S> = Pin<Box<dyn Future<Output = S> + 'static>>;

pub(crate) mod with_db_methods;
pub(crate) mod with_tx_methods;
pub(crate) mod without_db_methods;
