pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;
pub(crate) mod sql_processor_context;
pub(crate) mod success;

use std::sync::Arc;

use apllodb_shared_components::{ApllodbError, ApllodbSessionError, ApllodbSessionResult, Session};
use apllodb_sql_parser::{apllodb_ast, ApllodbAst, ApllodbSqlParser};
use apllodb_storage_engine_interface::{
    StorageEngine, WithDbMethods, WithTxMethods, WithoutDbMethods,
};

use self::{
    ddl::DdlProcessor, modification::ModificationProcessor, query::QueryProcessor,
    sql_processor_context::SqlProcessorContext, success::SqlProcessorSuccess,
};

use crate::ast_translator::AstTranslator;

/// Processes SQL.
#[derive(Debug, new)]
pub struct SqlProcessor<Engine: StorageEngine> {
    context: Arc<SqlProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> SqlProcessor<Engine> {
    /// # Failures
    ///
    /// - [InvalidDatabaseDefinition](apllodb-shared-components::SqlState::InvalidDatabaseDefinition) when:
    ///   - requesting an operation that uses an open database with [SessionWithoutDb](apllodb-shared-components::SessionWithoutDb).
    /// - [FeatureNotSupported](apllodb-shared-components::SqlState::FeatureNotSupported) when:
    ///   - the sql should be processed properly but apllodb currently doesn't
    pub async fn run(
        &self,
        session: Session,
        sql: &str,
    ) -> ApllodbSessionResult<SqlProcessorSuccess> {
        let parser = ApllodbSqlParser::default();

        match parser.parse(sql) {
            Err(e) => Err(
                ApllodbSessionError::new(
                    ApllodbError::syntax_error(
                        format!("failed to parse SQL: {}", sql),
                        Box::new(e),
                    ),
                    session
                )
            ),
            Ok(ApllodbAst(command)) => match session {
                Session::WithTx(sess) => match command {
                    apllodb_ast::Command::CommitTransactionCommandVariant => {
                        let session = self.context.engine.with_tx().commit_transaction(sess).await?;
                        Ok(SqlProcessorSuccess::TransactionEndRes {session})
                    }
                    apllodb_ast::Command::AbortTransactionCommandVariant => {
                        let session = self.context.engine.with_tx().abort_transaction(sess).await?;
                        Ok(SqlProcessorSuccess::TransactionEndRes {session})
                    }

                    apllodb_ast::Command::AlterTableCommandVariant(_)
                    | apllodb_ast::Command::CreateTableCommandVariant(_)
                    | apllodb_ast::Command::DropTableCommandVariant(_) => {
                        let processor = self.ddl();
                        let sess = processor.run(sess, command).await?;
                        Ok(SqlProcessorSuccess::DdlRes { session: sess })
                    }
                    apllodb_ast::Command::DeleteCommandVariant(_)
                    | apllodb_ast::Command::InsertCommandVariant(_)
                    | apllodb_ast::Command::UpdateCommandVariant(_) => {
                        let processor = self.modification();
                        let sess = processor.run(sess, command).await?;
                        Ok(SqlProcessorSuccess::ModificationRes { session: sess })
                    }
                    apllodb_ast::Command::SelectCommandVariant(select_command) => {
                        let processor = self.query();
                        let (records, sess) = processor.run(sess, select_command).await?;
                        Ok(SqlProcessorSuccess::QueryRes {
                            session: sess,
                            records,
                        })
                    }
                    apllodb_ast::Command::CreateDatabaseCommandVariant(_) | apllodb_ast::Command::UseDatabaseCommandVariant(_) => {
                        Err(
                            ApllodbSessionError::new(
                                ApllodbError::feature_not_supported(
                            format!("cannot process the following SQL (database: {:?}, transaction: open): {}", sess.get_db_name(), sql),
                        ), Session::from(sess)))
                    }
                    apllodb_ast::Command::BeginTransactionCommandVariant => {
                        Err(
                            ApllodbSessionError::new(
                            ApllodbError::invalid_transaction_state(
                            format!("transaction is already open (database: {:?}, transaction: open): {}",  sess.get_db_name(), sql),
                        ), Session::from(sess)
                    ))
                    }
                },
                Session::WithDb(sess) => match command {
                    apllodb_ast::Command::BeginTransactionCommandVariant => {
                        let session = self.context.engine.with_db().begin_transaction(sess).await?;
                        Ok(SqlProcessorSuccess::BeginTransactionRes {session})
                    }

                    apllodb_ast::Command::AlterTableCommandVariant(_)
                    | apllodb_ast::Command::CreateTableCommandVariant(_)
                    | apllodb_ast::Command::DropTableCommandVariant(_)
                    | apllodb_ast::Command::DeleteCommandVariant(_)
                    | apllodb_ast::Command::InsertCommandVariant(_)
                    | apllodb_ast::Command::UpdateCommandVariant(_)
                    | apllodb_ast::Command::SelectCommandVariant(_) => {
                        // TODO auto-commit feature here?
                        Err(ApllodbSessionError::new(
                            ApllodbError::feature_not_supported("auto-commit is not supported currently"),
                            Session::from(sess)
                        ))
                    }
                    apllodb_ast::Command::CreateDatabaseCommandVariant(_) | apllodb_ast::Command::UseDatabaseCommandVariant(_) => {
                        Err(
                            ApllodbSessionError::new(
                            ApllodbError::feature_not_supported(
                            format!("cannot process the following SQL (database: {:?}, transaction: none): {}", sess.database_name(), sql),
                        ), Session::from(sess)))
                    }

                    apllodb_ast::Command::CommitTransactionCommandVariant |apllodb_ast::Command::AbortTransactionCommandVariant => {
                        Err(
                            ApllodbSessionError::new(
                                ApllodbError::invalid_transaction_state(
                            format!("transaction is not open (database: {:?}, transaction: none): {}", sess.database_name(), sql),
                        ), Session::from(sess)))
                    }
                },
                Session::WithoutDb(sess) => match command {
                    apllodb_ast::Command::CreateDatabaseCommandVariant(cmd) => {
                        match AstTranslator::database_name(cmd.database_name) {
                            Ok(database_name) => {
                                let session = self.context.engine
                                .without_db()
                                .create_database(Session::from(sess), database_name)
                                .await?;
                            Ok(SqlProcessorSuccess::CreateDatabaseRes { session })
                            }
                            Err(e) => Err(ApllodbSessionError::new(e, Session::from(sess)))
                        }

                    }
                    apllodb_ast::Command::UseDatabaseCommandVariant(cmd) => {
                      match AstTranslator::database_name(cmd.database_name) {
                          Ok(database_name)=>{let session = self.context.engine
                            .without_db()
                            .use_database(sess, database_name)
                            .await?;
                        Ok(SqlProcessorSuccess::UseDatabaseRes { session })}
                        Err(e) => Err(ApllodbSessionError::new(e, Session::from(sess)))
                      }

                    }

                    apllodb_ast::Command::BeginTransactionCommandVariant
                    |apllodb_ast::Command::AbortTransactionCommandVariant
                    | apllodb_ast::Command::CommitTransactionCommandVariant
                    | apllodb_ast::Command::AlterTableCommandVariant(_)
                    | apllodb_ast::Command::CreateTableCommandVariant(_)
                    | apllodb_ast::Command::DropTableCommandVariant(_)
                    | apllodb_ast::Command::DeleteCommandVariant(_)
                    | apllodb_ast::Command::InsertCommandVariant(_)
                    | apllodb_ast::Command::UpdateCommandVariant(_)
                    | apllodb_ast::Command::SelectCommandVariant(_) => Err(
                        ApllodbSessionError::new(
                        ApllodbError::connection_exception_database_not_open(
                        format!("this command requires open database: {}", sql),
                    ), Session::from(sess))),
                },
            },
        }
    }

    fn ddl(&self) -> DdlProcessor<Engine> {
        DdlProcessor::new(self.context.clone())
    }

    fn modification(&self) -> ModificationProcessor<Engine> {
        ModificationProcessor::new(self.context.clone())
    }

    fn query(&self) -> QueryProcessor<Engine> {
        QueryProcessor::new(self.context.clone())
    }
}
