pub(crate) mod rec;
pub(crate) mod rec_iter;

use apllodb_shared_components::{Session, SessionWithDb, SessionWithTx};

use crate::RecIter;

/// Successful response from apllodb-server's [command()](crate::ApllodbServer::command).
#[derive(Debug)]
pub enum ApllodbCommandSuccess {
    QueryResponse {
        session: SessionWithTx,
        records: RecIter,
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
