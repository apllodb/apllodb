use apllodb_shared_components::{RecordIterator, Session, SessionWithTx};
use serde::{Deserialize, Serialize};

/// Successful response from apllodb-server
#[derive(Debug, Serialize, Deserialize)]
pub enum ApllodbSuccess {
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
    DatabaseResponse {
        session: Session,
    },
}
