pub(crate) mod with_db;
pub(crate) mod without_db;

use crate::{Database, Transaction};

use self::{with_db::SessionWithDb, without_db::SessionWithoutDb};

/// Session information.
///
/// Initially created from database client and used through server, sql-processor, and storage-engine.
///
/// A session holds these information:
///
/// - Open database (0/1)
/// - Beginning transaction (0/1 if a database is open; 0 if any database isn't open)
///
/// Only storage-engine is assumed to have direct access to database and transaction.
/// Other components create/modify/get database and transaction through session.
///
/// Note that session is free from physical connection implementation.
/// Therefore, for example, client-server's transport is independent from Session and can be any of TCP, direct method call, and so on.
#[derive(Hash, Debug)]
pub enum Session<Db: Database, Tx: Transaction> {
    /// Session with open database.
    WithDb(SessionWithDb<Db, Tx>),

    /// Session without open database.
    WithoutDb(SessionWithoutDb),
}
