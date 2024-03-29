use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, ApllodbSessionError, ApllodbSessionResult, Session, SessionWithTx,
};
use apllodb_sql_parser::apllodb_ast::{
    AlterTableCommand, Command, CreateTableCommand, TableElement,
};
use apllodb_storage_engine_interface::{
    AlterTableAction, ColumnDefinition, StorageEngine, TableConstraintKind, TableConstraints,
    TableName, WithTxMethods,
};

use crate::ast_translator::AstTranslator;

use super::sql_processor_context::SqlProcessorContext;

/// Processes DDL command.
#[derive(Clone, Debug, new)]
pub(crate) struct DdlProcessor<Engine: StorageEngine> {
    context: Arc<SqlProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> DdlProcessor<Engine> {
    /// Executes DDL command.
    pub async fn run(
        &self,
        session: SessionWithTx,
        command: Command,
    ) -> ApllodbSessionResult<SessionWithTx> {
        match command {
            Command::CreateTableCommandVariant(cc) => match self.run_helper_create_table(cc) {
                Ok((table_name, table_constraints, column_definitions)) => {
                    self.context
                        .engine
                        .with_tx()
                        .create_table(session, table_name, table_constraints, column_definitions)
                        .await
                }
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            },
            Command::AlterTableCommandVariant(ac) => match self.run_helper_alter_table(ac) {
                Ok((table_name, action)) => {
                    self.context
                        .engine
                        .with_tx()
                        .alter_table(session, table_name, action)
                        .await
                }
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            },
            _ => Err(ApllodbSessionError::new(
                ApllodbError::feature_not_supported(
                    "only CREATE TABLE / ALTER TABLE are supported for DDL currently",
                ),
                Session::from(session),
            )),
        }
    }

    fn run_helper_create_table(
        &self,
        command: CreateTableCommand,
    ) -> ApllodbResult<(TableName, TableConstraints, Vec<ColumnDefinition>)> {
        let table_name = AstTranslator::table_name(command.table_name)?;

        let column_definitions: Vec<ColumnDefinition> = command
            .table_elements
            .as_vec()
            .iter()
            .filter_map(|table_element| {
                if let TableElement::ColumnDefinitionVariant(cd) = table_element {
                    Some(cd)
                } else {
                    None
                }
            })
            .map(|cd| AstTranslator::column_definition(cd.clone()))
            .collect::<ApllodbResult<_>>()?;

        let table_constraints: Vec<TableConstraintKind> = command
            .table_elements
            .as_vec()
            .iter()
            .filter_map(|table_element| {
                if let TableElement::TableConstraintVariant(tc) = table_element {
                    Some(tc)
                } else {
                    None
                }
            })
            .map(|tc| AstTranslator::table_constraint(tc.clone()))
            .collect::<ApllodbResult<_>>()?;

        Ok((
            table_name,
            TableConstraints::new(table_constraints)?,
            column_definitions,
        ))
    }

    fn run_helper_alter_table(
        &self,
        command: AlterTableCommand,
    ) -> ApllodbResult<(TableName, AlterTableAction)> {
        let table_name = AstTranslator::table_name(command.table_name)?;

        let ast_actions = command.actions.into_vec();
        let ast_action = if ast_actions.len() > 1 {
            Err(ApllodbError::feature_not_supported(
                "ALTER TABLE does not support multiple actions currently",
            ))
        } else {
            Ok(ast_actions
                .first()
                .expect("NonEmptyVec assures first element")
                .clone())
        }?;

        let action = AstTranslator::alter_table_action(ast_action)?;

        Ok((table_name, action))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::DdlProcessor;
    use crate::sql_processor::sql_processor_context::SqlProcessorContext;
    use apllodb_shared_components::{ApllodbResult, SqlType};
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::{
        test_support::{default_mock_engine, test_models::People, MockWithTxMethods},
        ColumnConstraints, ColumnDataType, ColumnDefinition, TableConstraintKind, TableConstraints,
        TableName,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum {
        in_create_table_sql: String,
        expected_table_name: TableName,
        expected_table_constraints: Vec<TableConstraintKind>,
        expected_column_definitions: Vec<ColumnDefinition>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_ddl_processor_with_sql() -> ApllodbResult<()> {
        let parser = ApllodbSqlParser::default();

        fn test_data() -> Vec<TestDatum> {
            vec![TestDatum::new(
                "
            CREATE TABLE people (
                id INTEGER, 
                age INTEGER,
                PRIMARY KEY (id)
            )"
                .to_string(),
                People::table_name(),
                vec![TableConstraintKind::PrimaryKey {
                    column_names: vec![People::tc_id().as_column_name().clone()],
                }],
                vec![
                    ColumnDefinition::new(
                        ColumnDataType::new(
                            People::tc_id().as_column_name().clone(),
                            SqlType::integer(),
                            true,
                        ),
                        ColumnConstraints::default(),
                    ),
                    ColumnDefinition::new(
                        ColumnDataType::new(
                            People::tc_age().as_column_name().clone(),
                            SqlType::integer(),
                            true,
                        ),
                        ColumnConstraints::default(),
                    ),
                ],
            )]
        }

        for test_datum in test_data().into_iter() {
            let sql = test_datum.in_create_table_sql.clone();

            log::debug!("testing with SQL: {}", &sql);

            // mocking create_table()
            let mut engine = default_mock_engine();
            engine.expect_with_tx().returning(move || {
                let test_datum = test_datum.clone();

                let mut with_tx = MockWithTxMethods::new();
                with_tx
                    .expect_create_table()
                    .with(
                        always(),
                        eq(test_datum.expected_table_name),
                        eq(TableConstraints::new(test_datum.expected_table_constraints).unwrap()),
                        eq(test_datum.expected_column_definitions),
                    )
                    .returning(|session, _, _, _| async { Ok(session) }.boxed_local());
                with_tx
            });

            let context = Arc::new(SqlProcessorContext::new(engine));

            let ast = parser.parse(&sql).unwrap();
            DdlProcessor::run_directly(context.clone(), ast.0).await?;
        }

        Ok(())
    }
}
