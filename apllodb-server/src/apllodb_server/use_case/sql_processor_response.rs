use apllodb_sql_processor::SQLProcessorSuccess;

use crate::ApllodbCommandSuccess;

pub(in crate::apllodb_server::use_case) fn to_server_resp(
    sql_processor_success: SQLProcessorSuccess,
) -> ApllodbCommandSuccess {
    match sql_processor_success {
        SQLProcessorSuccess::QueryRes { session, records } => {
            ApllodbCommandSuccess::QueryResponse { session, records }
        }
        SQLProcessorSuccess::ModificationRes { session } => {
            ApllodbCommandSuccess::ModificationResponse { session }
        }
        SQLProcessorSuccess::DDLRes { session } => ApllodbCommandSuccess::DDLResponse { session },
        SQLProcessorSuccess::CreateDatabaseRes { session } => {
            ApllodbCommandSuccess::CreateDatabaseResponse { session }
        }
        SQLProcessorSuccess::UseDatabaseRes { session } => {
            ApllodbCommandSuccess::UseDatabaseResponse { session }
        }
        SQLProcessorSuccess::BeginTransactionRes { session } => {
            ApllodbCommandSuccess::BeginTransactionResponse { session }
        }
        SQLProcessorSuccess::TransactionEndRes { session } => {
            ApllodbCommandSuccess::TransactionEndResponse { session }
        }
    }
}
