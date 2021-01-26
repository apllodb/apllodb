pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;
pub(crate) mod success;

use std::rc::Rc;

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, Session};
use apllodb_sql_parser::{apllodb_ast, ApllodbAst, ApllodbSqlParser};
use apllodb_storage_engine_interface::StorageEngine;

use self::{
    ddl::DDLProcessor, modification::ModificationProcessor, query::QueryProcessor,
    success::SQLProcessorSuccess,
};

/// Processes SQL.
#[derive(Clone, Debug, new)]
pub struct SQLProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> SQLProcessor<Engine> {
    /// # Failures
    ///
    /// - [InvalidDatabaseDefinition](apllodb-shared-components::ApllodbErrorKind::InvalidDatabaseDefinition) when:
    ///   - requesting an operation that uses an open database with [SessionWithoutDb](apllodb-shared-components::SessionWithoutDb).
    pub async fn run(&self, session: Session, sql: &str) -> ApllodbResult<SQLProcessorSuccess> {
        let parser = ApllodbSqlParser::default();

        match parser.parse(sql) {
            Err(e) => Err(ApllodbError::new(
                ApllodbErrorKind::SyntaxError,
                format!("failed to parse SQL: {}", sql),
                Some(Box::new(e)),
            )),
            Ok(ApllodbAst(command)) => match session {
                Session::WithTx(sess) => match command {
                    apllodb_ast::Command::AlterTableCommandVariant(_)
                    | apllodb_ast::Command::CreateTableCommandVariant(_)
                    | apllodb_ast::Command::DropTableCommandVariant(_) => {
                        let processor = self.ddl();
                        let sess = processor.run(sess, command).await?;
                        Ok(SQLProcessorSuccess::DDLRes { session: sess })
                    }
                    apllodb_ast::Command::DeleteCommandVariant(_)
                    | apllodb_ast::Command::InsertCommandVariant(_)
                    | apllodb_ast::Command::UpdateCommandVariant(_) => {
                        let processor = self.modification();
                        let sess = processor.run(sess, command).await?;
                        Ok(SQLProcessorSuccess::ModificationRes { session: sess })
                    }
                    apllodb_ast::Command::SelectCommandVariant(select_command) => {
                        let processor = self.query();
                        let (records, sess) = processor.run(sess, select_command).await?;
                        Ok(SQLProcessorSuccess::QueryRes {
                            session: sess,
                            records,
                        })
                    }
                },
                Session::WithDb(_) => match command {
                    apllodb_ast::Command::AlterTableCommandVariant(_)
                    | apllodb_ast::Command::CreateTableCommandVariant(_)
                    | apllodb_ast::Command::DropTableCommandVariant(_)
                    | apllodb_ast::Command::DeleteCommandVariant(_)
                    | apllodb_ast::Command::InsertCommandVariant(_)
                    | apllodb_ast::Command::UpdateCommandVariant(_)
                    | apllodb_ast::Command::SelectCommandVariant(_) => {
                        // TODO auto-commit feature here?
                        todo!()
                    }
                },
                Session::WithoutDb(_) => match command {
                    apllodb_ast::Command::AlterTableCommandVariant(_)
                    | apllodb_ast::Command::CreateTableCommandVariant(_)
                    | apllodb_ast::Command::DropTableCommandVariant(_)
                    | apllodb_ast::Command::DeleteCommandVariant(_)
                    | apllodb_ast::Command::InsertCommandVariant(_)
                    | apllodb_ast::Command::UpdateCommandVariant(_)
                    | apllodb_ast::Command::SelectCommandVariant(_) => Err(ApllodbError::new(
                        ApllodbErrorKind::InvalidDatabaseDefinition,
                        format!("this command requires open database: {}", sql),
                        None,
                    )),
                },
            },
        }
    }

    fn ddl(&self) -> DDLProcessor<Engine> {
        DDLProcessor::new(self.engine.clone())
    }

    fn modification(&self) -> ModificationProcessor<Engine> {
        ModificationProcessor::new(self.engine.clone())
    }

    fn query(&self) -> QueryProcessor<Engine> {
        QueryProcessor::new(self.engine.clone())
    }
}
