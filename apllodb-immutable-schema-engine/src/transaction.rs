use apllodb_storage_manager_interface::TxCtxLike;

/// Simple ACID transaction implementation for SERIALIZABLE isolation level.
pub(crate) struct TxCtx;

impl TxCtxLike for TxCtx {
    fn begin() -> apllodb_shared_components::error::ApllodbResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    fn commit(self) -> apllodb_shared_components::error::ApllodbResult<()> {
        todo!()
    }
    fn abort(self) -> apllodb_shared_components::error::ApllodbResult<()> {
        todo!()
    }
}
