use apllodb_immutable_schema_engine::{ApllodbImmutableSchemaEngine, ApllodbImmutableSchemaTx};
use apllodb_rpc_interface::ApllodbRpcSuccess;
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName};
use apllodb_sql_parser::{apllodb_ast, ApllodbAst, ApllodbSqlParser};
use apllodb_sql_processor::{DDLProcessor, ModificationProcessor, QueryProcessor};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};
use chrono::Local;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(in crate::apllodb_server) struct UseCase;

impl UseCase {
    pub(in crate::apllodb_server) fn command(sql: &str) -> ApllodbResult<ApllodbRpcSuccess> {
        let parser = ApllodbSqlParser::new();

        let database_name_from_client = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

        let mut db = ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(
            database_name_from_client,
        )?)?;
        let tx = ApllodbImmutableSchemaTx::begin(&mut db)?;

        let ret: ApllodbResult<ApllodbRpcSuccess> = match parser.parse(sql) {
            Err(e) => Err(ApllodbError::new(
                ApllodbErrorKind::SyntaxError,
                format!("failed to parse SQL: {}", sql),
                Some(Box::new(e)),
            )),
            Ok(ApllodbAst(command)) => match command {
                apllodb_ast::Command::AlterTableCommandVariant(_)
                | apllodb_ast::Command::CreateTableCommandVariant(_)
                | apllodb_ast::Command::DropTableCommandVariant(_) => {
                    let processor = DDLProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                    processor.run(command)?;
                    Ok(ApllodbRpcSuccess::DDLResponse)
                }
                apllodb_ast::Command::DeleteCommandVariant(_)
                | apllodb_ast::Command::InsertCommandVariant(_)
                | apllodb_ast::Command::UpdateCommandVariant(_) => {
                    let processor =
                        ModificationProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                    processor.run(command)?;
                    Ok(ApllodbRpcSuccess::ModificationResponse)
                }
                apllodb_ast::Command::SelectCommandVariant(select_command) => {
                    let processor = QueryProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                    let records = processor.run(select_command)?;
                    Ok(ApllodbRpcSuccess::QueryResponse { records })
                }
            },
        };

        tx.commit()?;

        ret
    }
}
