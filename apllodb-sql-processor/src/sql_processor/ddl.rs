use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, AstTranslator, ColumnDefinition,
    Session, SessionWithTx, TableConstraintKind, TableConstraints, TableName,
};
use apllodb_sql_parser::apllodb_ast::{Command, CreateTableCommand, TableElement};
use apllodb_storage_engine_interface::{StorageEngine, WithTxMethods};

use super::sql_processor_context::SQLProcessorContext;

/// Processes DDL command.
#[derive(Clone, Debug, new)]
pub(crate) struct DDLProcessor<Engine: StorageEngine> {
    context: Arc<SQLProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> DDLProcessor<Engine> {
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
            _ => unimplemented!(),
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::DDLProcessor;
    use crate::sql_processor::sql_processor_context::SQLProcessorContext;
    use apllodb_shared_components::{
        test_support::test_models::People, ApllodbResult, ColumnConstraints, ColumnDataType,
        ColumnDefinition, SqlType, TableConstraintKind, TableConstraints, TableName,
    };
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, session_with_tx, MockWithTxMethods,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum<'test> {
        in_create_table_sql: &'test str,
        expected_table_name: TableName,
        expected_table_constraints: Vec<TableConstraintKind>,
        expected_column_definitions: Vec<ColumnDefinition>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_ddl_processor_with_sql() -> ApllodbResult<()> {
        let parser = ApllodbSqlParser::default();

        static TEST_DATA: Lazy<Box<[TestDatum]>> = Lazy::new(|| {
            vec![TestDatum::new(
                "
            CREATE TABLE people (
                id INTEGER, 
                age INTEGER,
                PRIMARY KEY (id)
            )",
                People::table_name(),
                vec![TableConstraintKind::PrimaryKey {
                    column_names: vec![People::ffr_id().as_column_name().clone()],
                }],
                vec![
                    ColumnDefinition::new(
                        ColumnDataType::new(
                            People::ffr_id().as_column_name().clone(),
                            SqlType::integer(),
                            true,
                        ),
                        ColumnConstraints::default(),
                    ),
                    ColumnDefinition::new(
                        ColumnDataType::new(
                            People::ffr_age().as_column_name().clone(),
                            SqlType::integer(),
                            true,
                        ),
                        ColumnConstraints::default(),
                    ),
                ],
            )]
            .into_boxed_slice()
        });

        for test_datum in TEST_DATA.iter() {
            log::debug!("testing with SQL: {}", test_datum.in_create_table_sql);

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

            let ast = parser.parse(test_datum.in_create_table_sql).unwrap();
            let session = session_with_tx(&engine).await?;
            let processor = DDLProcessor::new(Arc::new(SQLProcessorContext::new(engine)));
            processor.run(session, ast.0).await?;
        }

        Ok(())
    }
}
