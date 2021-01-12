use apllodb_shared_components::RecordIterator;
use serde::{Deserialize, Serialize};

/// Successful result from [ApllodbSQLProcessor](crate::ApllodbSQLProcessor).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SQLProcessorSuccess {
    QueryRes { records: RecordIterator },
    ModificationRes,
    DDLRes,
}
