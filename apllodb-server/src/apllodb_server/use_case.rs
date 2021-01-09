use apllodb_immutable_schema_engine::{
    ApllodbImmutableSchemaDDL, ApllodbImmutableSchemaDML, ApllodbImmutableSchemaDb,
    ApllodbImmutableSchemaEngine, ApllodbImmutableSchemaTx,
};
use apllodb_rpc_interface::ApllodbRpcSuccess;
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, Database, DatabaseName, Transaction,
};
use apllodb_sql_parser::{apllodb_ast, ApllodbAst, ApllodbSqlParser};
use apllodb_sql_processor::{DDLProcessor, ModificationProcessor, QueryProcessor};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(in crate::apllodb_server) struct UseCase;

impl UseCase {
    pub(in crate::apllodb_server) fn command(
        db: DatabaseName,
        sql: &str,
    ) -> ApllodbResult<ApllodbRpcSuccess> {
        let parser = ApllodbSqlParser::new();

        let ddl = ApllodbImmutableSchemaDDL::default();
        let dml = ApllodbImmutableSchemaDML::default();

        let mut db = ApllodbImmutableSchemaDb::use_database(db.clone())?;
        let mut tx = ApllodbImmutableSchemaTx::begin(&mut db)?;

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
                    let processor = DDLProcessor::<ApllodbImmutableSchemaEngine>::new(&ddl);
                    processor.run(&mut tx, command)?;
                    Ok(ApllodbRpcSuccess::DDLResponse)
                }
                apllodb_ast::Command::DeleteCommandVariant(_)
                | apllodb_ast::Command::InsertCommandVariant(_)
                | apllodb_ast::Command::UpdateCommandVariant(_) => {
                    let processor =
                        ModificationProcessor::<ApllodbImmutableSchemaEngine>::new(&dml);
                    processor.run(&mut tx, command)?;
                    Ok(ApllodbRpcSuccess::ModificationResponse)
                }
                apllodb_ast::Command::SelectCommandVariant(select_command) => {
                    let processor = QueryProcessor::<ApllodbImmutableSchemaEngine>::new(&dml);
                    let records = processor.run(&mut tx, select_command)?;
                    Ok(ApllodbRpcSuccess::QueryResponse { records })
                }
            },
        };

        tx.commit()?;

        ret
    }
}
