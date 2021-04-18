use apllodb_shared_components::{Session, SessionWithDb, SessionWithTx};
use apllodb_sql_processor::Records;

/// Successful response from apllodb-server's [command()](crate::ApllodbServer::command).
#[derive(Debug)]
pub enum ApllodbCommandSuccess {
    QueryResponse {
        session: SessionWithTx,
        records: Records,
    },
    ModificationResponse {
        session: SessionWithTx,
    },
    DdlResponse {
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
