use std::collections::HashMap;

use apllodb_shared_components::SessionId;
use generational_arena::{Arena, Index};

use super::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx};

use once_cell::sync::Lazy;

// FIXME Consider sharding by SessionId to avoid writer contention using something like dashmap.
// see: <https://tokio.rs/tokio/tutorial/shared-state#tasks-threads-and-contention>
static SQ_POOL: Lazy<SqliteResourcePool<'static>> = Lazy::new(|| SqliteResourcePool::default());

/// rusqlite's Connection and Transaction pool.
///
/// Each resource is accessible via [SessionId](apllodb-shared-components::SessionId).
#[derive(Debug, Default)]
pub(crate) struct SqliteResourcePool<'sqcn> {
    pub(crate) db_arena: Arena<SqliteDatabase>,
    pub(crate) tx_arena: Arena<SqliteTx<'sqcn>>,

    pub(crate) sess_db: HashMap<SessionId, Index>,
    pub(crate) sess_tx: HashMap<SessionId, Index>,
}

impl SqliteResourcePool<'_> {
    pub(crate) fn register_db(sid: &SessionId, db: SqliteDatabase) {
        let db_idx = SQ_POOL.db_arena.insert(db);
        SQ_POOL.sess_db.insert(sid.clone(), db_idx);
    }
}
