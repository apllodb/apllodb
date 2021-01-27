use apllodb_sql_processor::SQLProcessorSuccess;

use crate::ApllodbSuccess;

pub(in crate::apllodb_server::use_case) fn to_server_resp(
    sql_processor_success: SQLProcessorSuccess,
) -> ApllodbSuccess {
    match sql_processor_success {
        SQLProcessorSuccess::QueryRes { session, records } => {
            ApllodbSuccess::QueryResponse { session, records }
        }
        SQLProcessorSuccess::ModificationRes { session } => {
            ApllodbSuccess::ModificationResponse { session }
        }
        SQLProcessorSuccess::DDLRes { session } => ApllodbSuccess::DDLResponse { session },
        SQLProcessorSuccess::CreateDatabaseRes { session } => {
            ApllodbSuccess::CreateDatabaseResponse { session }
        }
        SQLProcessorSuccess::UseDatabaseRes { session } => {
            ApllodbSuccess::UseDatabaseResponse { session }
        }
        SQLProcessorSuccess::BeginTransactionRes { session } => {
            ApllodbSuccess::BeginTransactionResponse { session }
        }
        SQLProcessorSuccess::TransactionEndRes { session } => {
            ApllodbSuccess::TransactionEndResponse { session }
        }
    }
}
