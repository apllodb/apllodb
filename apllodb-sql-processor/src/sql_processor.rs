pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;
pub(crate) mod success;

use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbSessionError, ApllodbSessionResult, AstTranslator,
    Session,
};
use apllodb_sql_parser::{apllodb_ast, ApllodbAst, ApllodbSqlParser};
use apllodb_storage_engine_interface::{
    StorageEngine, WithDbMethods, WithTxMethods, WithoutDbMethods,
};

use self::{
    ddl::DDLProcessor,
    modification::ModificationProcessor,
    query::{
        query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository,
        QueryProcessor,
    },
    success::SQLProcessorSuccess,
};

/// Processes SQL.
#[derive(Debug)]
pub struct SQLProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
    node_repo: Arc<RwLock<QueryPlanNodeRepository>>,
}

impl<Engine: StorageEngine> SQLProcessor<Engine> {
    /// Constructor
    pub fn new(engine: Rc<Engine>) -> Self {
        Self {
            engine,
            node_repo: Arc::new(RwLock::new(QueryPlanNodeRepository::default())),
        }
    }

    /// # Failures
    ///
    /// - [InvalidDatabaseDefinition](apllodb-shared-components::ApllodbErrorKind::InvalidDatabaseDefinition) when:
    ///   - requesting an operation that uses an open database with [SessionWithoutDb](apllodb-shared-components::SessionWithoutDb).
    /// - [FeatureNotSupported](apllodb-shared-components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - the sql should be processed properly but apllodb currently doesn't
    pub async fn run(
        &self,
        session: Session,
        sql: &str,
    ) -> ApllodbSessionResult<SQLProcessorSuccess> {
        let parser = ApllodbSqlParser::default();

        match parser.parse(sql) {
            Err(e) => Err(
                ApllodbSessionError::new(
                ApllodbError::new(
                ApllodbErrorKind::SyntaxError,
                format!("failed to parse SQL: {}", sql),
                Some(Box::new(e)),
            ), session)),
            Ok(ApllodbAst(command)) => match session {
                Session::WithTx(sess) => match command {
                    apllodb_ast::Command::CommitTransactionCommandVariant => {
                        let session = self.engine.with_tx().commit_transaction(sess).await?;
                        Ok(SQLProcessorSuccess::TransactionEndRes {session})
                    }
                    apllodb_ast::Command::AbortTransactionCommandVariant => {
                        let session = self.engine.with_tx().abort_transaction(sess).await?;
                        Ok(SQLProcessorSuccess::TransactionEndRes {session})
                    }

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
                    apllodb_ast::Command::CreateDatabaseCommandVariant(_) | apllodb_ast::Command::UseDatabaseCommandVariant(_) => {
                        Err(
                            ApllodbSessionError::new(
                                ApllodbError::new(
                            ApllodbErrorKind::FeatureNotSupported,
                            format!("cannot process the following SQL (database: {:?}, transaction: open): {}", sess.get_db_name(), sql),
                            None,
                        ), Session::from(sess)))
                    }
                    apllodb_ast::Command::BeginTransactionCommandVariant => {
                        Err(
                            ApllodbSessionError::new(
                            ApllodbError::new(
                            ApllodbErrorKind::InvalidTransactionState,
                            format!("transaction is already open (database: {:?}, transaction: open): {}",  sess.get_db_name(), sql),
                            None,
                        ), Session::from(sess)
                    ))
                    }
                },
                Session::WithDb(sess) => match command {
                    apllodb_ast::Command::BeginTransactionCommandVariant => {
                        let session = self.engine.with_db().begin_transaction(sess).await?;
                        Ok(SQLProcessorSuccess::BeginTransactionRes {session})
                    }

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
                    apllodb_ast::Command::CreateDatabaseCommandVariant(_) | apllodb_ast::Command::UseDatabaseCommandVariant(_) => {
                        Err(
                            ApllodbSessionError::new(
                            ApllodbError::new(
                            ApllodbErrorKind::FeatureNotSupported,
                            format!("cannot process the following SQL (database: {:?}, transaction: none): {}", sess.database_name(), sql),
                            None,
                        ), Session::from(sess)))
                    }

                    apllodb_ast::Command::CommitTransactionCommandVariant |apllodb_ast::Command::AbortTransactionCommandVariant => {
                        Err(
                            ApllodbSessionError::new(
                                ApllodbError::new(
                            ApllodbErrorKind::InvalidTransactionState,
                            format!("transaction is not open (database: {:?}, transaction: none): {}", sess.database_name(), sql),
                            None,
                        ), Session::from(sess)))
                    }
                },
                Session::WithoutDb(sess) => match command {
                    apllodb_ast::Command::CreateDatabaseCommandVariant(cmd) => {
                        match AstTranslator::database_name(cmd.database_name) {
                            Ok(database_name) => {
                                let session = self.engine
                                .without_db()
                                .create_database(Session::from(sess), database_name)
                                .await?;
                            Ok(SQLProcessorSuccess::CreateDatabaseRes { session })
                            }
                            Err(e) => Err(ApllodbSessionError::new(e, Session::from(sess)))
                        }

                    }
                    apllodb_ast::Command::UseDatabaseCommandVariant(cmd) => {
                      match AstTranslator::database_name(cmd.database_name) {
                          Ok(database_name)=>{let session = self.engine
                            .without_db()
                            .use_database(sess, database_name)
                            .await?;
                        Ok(SQLProcessorSuccess::UseDatabaseRes { session })}
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
                        ApllodbError::new(
                        ApllodbErrorKind::InvalidDatabaseDefinition,
                        format!("this command requires open database: {}", sql),
                        None,
                    ), Session::from(sess))),
                },
            },
        }
    }

    fn ddl(&self) -> DDLProcessor<Engine> {
        DDLProcessor::new(self.engine.clone())
    }

    fn modification(&self) -> ModificationProcessor<Engine> {
        ModificationProcessor::new(self.engine.clone(), self.node_repo.clone())
    }

    fn query(&self) -> QueryProcessor<Engine> {
        QueryProcessor::new(self.engine.clone(), self.node_repo.clone())
    }
}
