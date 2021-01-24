use apllodb_rpc_interface::ApllodbSuccess;
use apllodb_sql_processor::SQLProcessorSuccess;

pub(in crate::apllodb_server::use_case) fn to_rpc_success(
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
    }
}
