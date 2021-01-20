use std::collections::HashMap;

use apllodb_shared_components::SessionId;
use generational_arena::{Arena, Index};

use super::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx};

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
    pub(crate) fn register_db(&mut self, sid: &SessionId, db: SqliteDatabase) {
        let db_idx = self.db_arena.insert(db);
        self.sess_db.insert(sid.clone(), db_idx);
    }
}