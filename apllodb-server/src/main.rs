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
use chrono::Local;

fn main() -> ApllodbResult<()> {
    let parser = ApllodbSqlParser::new();

    let database_name_from_client = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let mut db =
        ApllodbImmutableSchemaEngine::use_database(&DatabaseName::new(database_name_from_client)?)?;
    let tx = ApllodbImmutableSchemaTx::begin(&mut db)?;

    let sqls_from_client = vec![
        "CREATE TABLE people (id INTEGER, age SMALLINT, PRIMARY KEY (id))",
        "INSERT INTO people (id, age) VALUES (1, 13)",
        "INSERT INTO people (id, age) VALUES (2, 70)",
        "INSERT INTO people (id, age) VALUES (3, 35)",
        "SELECT id, age FROM people",
    ];

    for sql in sqls_from_client {
        match parser.parse(sql) {
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
                    processor.run(command)
                }
                apllodb_ast::Command::DeleteCommandVariant(_)
                | apllodb_ast::Command::InsertCommandVariant(_)
                | apllodb_ast::Command::UpdateCommandVariant(_) => {
                    let processor =
                        ModificationProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                    processor.run(command)
                }
                apllodb_ast::Command::SelectCommandVariant(select_command) => {
                    let processor = QueryProcessor::<'_, ApllodbImmutableSchemaEngine>::new(&tx);
                    let records = processor.run(select_command)?;
                    // TODO return records to client
                    log::info!("SELECT result: {:#?}", records);
                    Ok(())
                }
            },
        }?;
    }

    tx.commit()
}
