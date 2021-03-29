use apllodb_sql_processor::SqlProcessorSuccess;

use crate::{ApllodbCommandSuccess, RecIter};

pub(in crate::apllodb_server::use_case) fn to_server_resp(
    sql_processor_success: SqlProcessorSuccess,
) -> ApllodbCommandSuccess {
    match sql_processor_success {
        SqlProcessorSuccess::QueryRes { session, records } => {
            ApllodbCommandSuccess::QueryResponse {
                session,
                records: RecIter::from(records),
            }
        }
        SqlProcessorSuccess::ModificationRes { session } => {
            ApllodbCommandSuccess::ModificationResponse { session }
        }
        SqlProcessorSuccess::DDLRes { session } => ApllodbCommandSuccess::DDLResponse { session },
        SqlProcessorSuccess::CreateDatabaseRes { session } => {
            ApllodbCommandSuccess::CreateDatabaseResponse { session }
        }
        SqlProcessorSuccess::UseDatabaseRes { session } => {
            ApllodbCommandSuccess::UseDatabaseResponse { session }
        }
        SqlProcessorSuccess::BeginTransactionRes { session } => {
            ApllodbCommandSuccess::BeginTransactionResponse { session }
        }
        SqlProcessorSuccess::TransactionEndRes { session } => {
            ApllodbCommandSuccess::TransactionEndResponse { session }
        }
    }
}
