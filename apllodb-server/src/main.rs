#![deny(warnings, missing_debug_implementations)]

//! apllodb's server bin crate.
//!
//! This crate directly depends on apllodb-immutable-schema-engine currently, although apllodb's storage engine is designed to be plugable.
//! See <https://github.com/darwin-education/apllodb/issues/47#issuecomment-753779450> for future plan.

use apllodb_immutable_schema_engine::{ApllodbImmutableSchemaEngine, ApllodbImmutableSchemaTx};
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName};
use apllodb_sql_parser::{apllodb_ast, ApllodbAst, ApllodbSqlParser};
use apllodb_sql_processor::{DDLProcessor, ModificationProcessor, QueryProcessor};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

fn main() -> ApllodbResult<()> {
    let parser = ApllodbSqlParser::new();

    let database_name_from_client = "people_db";

    let mut db =
        ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(database_name_from_client)?)?;
    let tx = ApllodbImmutableSchemaTx::begin(&mut db)?;

    let sql_from_client = "CREATE TABLE people (id INTEGER, age INTEGER)";

    match parser.parse(sql_from_client) {
        Err(e) => Err(ApllodbError::new(
            ApllodbErrorKind::SyntaxError,
            format!("failed to parse SQL: {}", sql_from_client),
            Some(Box::new(e)),
        )),
        Ok(ApllodbAst(command)) => match command {
            apllodb_ast::Command::AlterTableCommandVariant(_)
            | apllodb_ast::Command::CreateTableCommandVariant(_)
            | apllodb_ast::Command::DropTableCommandVariant(_) => {
                let processor = DDLProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                processor.run(command)
            }
            apllodb_ast::Command::DeleteCommandVariant(_)
            | apllodb_ast::Command::InsertCommandVariant(_)
            | apllodb_ast::Command::UpdateCommandVariant(_) => {
                let processor = ModificationProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                processor.run(command)
            }
            apllodb_ast::Command::SelectCommandVariant(select_command) => {
                let processor = QueryProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                let records = processor.run(select_command)?;
                // TODO return records to client
                log::debug!("SELECT result: {:#?}", records);
                Ok(())
            }
        },
    }
}
