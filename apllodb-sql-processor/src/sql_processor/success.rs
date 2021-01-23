use apllodb_shared_components::{RecordIterator, SessionWithTx};
use serde::{Deserialize, Serialize};

/// Successful result from [ApllodbSQLProcessor](crate::ApllodbSQLProcessor).
#[derive(Debug, Serialize, Deserialize)]
pub enum SQLProcessorSuccess {
    QueryRes {
        session: SessionWithTx,
        records: RecordIterator,
    },
    ModificationRes {
        session: SessionWithTx,
    },
    DDLRes {
        session: SessionWithTx,
    },
}
