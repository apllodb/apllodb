use std::rc::Rc;

use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, SessionWithTx, TableConstraintKind, TableConstraints,
};
use apllodb_sql_parser::apllodb_ast::{Command, TableElement};
use apllodb_storage_engine_interface::{StorageEngine, WithTxMethods};

use crate::ast_translator::AstTranslator;

/// Processes DDL command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DDLProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> DDLProcessor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

    /// Executes DDL command.
    pub async fn run(
        &self,
        session: SessionWithTx,
        command: Command,
    ) -> ApllodbResult<SessionWithTx> {
        match command {
            Command::CreateTableCommandVariant(cc) => {
                let table_name = AstTranslator::table_name(cc.table_name)?;

                let column_definitions: Vec<ColumnDefinition> = cc
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
                    .map(|cd| AstTranslator::column_definition(cd.clone(), table_name.clone()))
                    .collect::<ApllodbResult<_>>()?;

                let table_constraints: Vec<TableConstraintKind> = cc
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

                let session = self
                    .engine
                    .with_tx()
                    .create_table(
                        session,
                        table_name,
                        TableConstraints::new(table_constraints)?,
                        column_definitions,
                    )
                    .await?;

                Ok(session)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use apllodb_shared_components::{
        ApllodbResult, ColumnConstraints, ColumnDataType, ColumnDefinition, SqlType,
        TableConstraintKind, TableConstraints, TableName,
    };
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, session_with_tx, test_models::People, MockWithTxMethods,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    use super::DDLProcessor;

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
                    column_names: vec![People::colref_id().as_column_name().clone()],
                }],
                vec![
                    ColumnDefinition::new(
                        ColumnDataType::new(People::colref_id(), SqlType::integer(), true),
                        ColumnConstraints::default(),
                    ),
                    ColumnDefinition::new(
                        ColumnDataType::new(People::colref_age(), SqlType::integer(), true),
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
            let processor = DDLProcessor::new(Rc::new(engine));
            processor.run(session, ast.0).await?;
        }

        Ok(())
    }
}
