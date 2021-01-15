use std::collections::HashMap;

use apllodb_shared_components::SessionId;
use generational_arena::{Arena, Index};

use super::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx};

/// rusqlite's Connection and Transaction pool.
///
/// Each resource is accessible via [SessionId](apllodb-shared-components::SessionId).
#[derive(Debug, Default)]
pub(crate) struct SqliteResourcePool<'sqcn> {
    db_arena: Arena<SqliteDatabase>,
    tx_arena: Arena<SqliteTx<'sqcn>>,

    sess_db: HashMap<SessionId, Index>,
    sess_tx: HashMap<SessionId, Index>,
}
