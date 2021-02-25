pub(crate) mod record_iterator;

use apllodb_shared_components::{Session, SessionWithDb, SessionWithTx};

use crate::RecordIterator;

/// Successful response from apllodb-server's [command()](crate::ApllodbServer::command).
#[derive(Debug)]
pub enum ApllodbCommandSuccess {
    QueryResponse {
        session: SessionWithTx,
        records: RecordIterator,
    },
    ModificationResponse {
        session: SessionWithTx,
    },
    DDLResponse {
        session: SessionWithTx,
    },
    CreateDatabaseResponse {
        session: Session,
    },
    UseDatabaseResponse {
        session: SessionWithDb,
    },
    BeginTransactionResponse {
        session: SessionWithTx,
    },
    TransactionEndResponse {
        session: SessionWithDb,
    },
}
