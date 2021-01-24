use apllodb_rpc_interface::ApllodbRpcSuccess;
use apllodb_sql_processor::SQLProcessorSuccess;

pub(in crate::apllodb_server::use_case) fn to_rpc_success(
    sql_processor_success: SQLProcessorSuccess,
) -> ApllodbRpcSuccess {
    match sql_processor_success {
        SQLProcessorSuccess::QueryRes { session, records } => {
            ApllodbRpcSuccess::QueryResponse { session, records }
        }
        SQLProcessorSuccess::ModificationRes { session } => {
            ApllodbRpcSuccess::ModificationResponse { session }
        }
        SQLProcessorSuccess::DDLRes { session } => ApllodbRpcSuccess::DDLResponse { session },
    }
}
