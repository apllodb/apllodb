use std::collections::HashMap;

use apllodb_shared_components::SessionId;
use generational_arena::{Arena, Index};

/// rusqlite's Connection and Transaction pool.
///
/// Each resource is accessible via [SessionId](apllodb-shared-components::SessionId).
#[derive(Debug, Default)]
pub(crate) struct SqliteResourcePool<'sqcn> {
    conn_arena: Arena<rusqlite::Connection>,
    tx_arena: Arena<rusqlite::Transaction<'sqcn>>,

    sess_conn: HashMap<SessionId, Index>,
    sess_tx: HashMap<SessionId, Index>,
}
