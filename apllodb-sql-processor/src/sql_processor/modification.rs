use std::{convert::TryFrom, rc::Rc};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, AstTranslator, ColumnName, Session,
    SessionWithTx, SqlValue, SqlValues,
};
use apllodb_sql_parser::apllodb_ast::{Command, InsertCommand};
use apllodb_storage_engine_interface::StorageEngine;

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
    LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf,
};

use self::{
    modification_executor::ModificationExecutor,
    modification_plan::{
        modification_plan_tree::{
            modification_plan_node::{InsertNode, ModificationPlanNode},
            ModificationPlanTree,
        },
        ModificationPlan,
    },
};

pub(crate) mod modification_executor;
pub(crate) mod modification_plan;

/// Processes ÎNSERT/UPDATE/DELETE command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ModificationProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> ModificationProcessor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

    /// Executes parsed INSERT/UPDATE/DELETE command.
    pub async fn run(
        &self,
        session: SessionWithTx,
        command: Command,
    ) -> ApllodbSessionResult<SessionWithTx> {
        match command {
            Command::InsertCommandVariant(ic) => match self.run_helper_insert(ic) {
                Ok(plan) => {
                    let executor = ModificationExecutor::new(self.engine.clone());
                    executor.run(session, plan).await
                }
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            },
            _ => unimplemented!(),
        }
    }

    fn run_helper_insert(&self, command: InsertCommand) -> ApllodbResult<ModificationPlan> {
        if command.alias.is_some() {
            unimplemented!();
        }

        let ast_table_name = command.table_name;
        let table_name = AstTranslator::table_name(ast_table_name.clone())?;

        let ast_column_names = command.column_names.into_vec();
        let column_names: Vec<ColumnName> = ast_column_names
            .into_iter()
            .map(AstTranslator::column_name)
            .collect::<ApllodbResult<_>>()?;
        let expressions = command.expressions.into_vec();

        if column_names.len() != expressions.len() {
            unimplemented!();
        }

        let constant_values: Vec<SqlValue> = expressions
            .into_iter()
            .map(|ast_expression| {
                let expression = AstTranslator::expression(ast_expression)?;
                SqlValue::try_from(expression)
            })
            .collect::<ApllodbResult<_>>()?;

        let insert_values = SqlValues::new(constant_values);

        let plan_node = ModificationPlanNode::Insert(InsertNode {
            table_name: table_name.clone(),
            child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                op: LeafPlanOperation::InsertValues {
                    table_name,
                    column_names,
                    values: vec![insert_values],
                },
            }),
        });

        Ok(ModificationPlan::new(ModificationPlanTree::new(plan_node)))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use apllodb_shared_components::{
        test_support::test_models::People, ApllodbResult, ColumnName, NNSqlValue, SqlValue,
        SqlValues, TableName,
    };
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, session_with_tx, MockWithTxMethods,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    use super::ModificationProcessor;

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum {
        in_insert_sql: &'static str,
        expected_insert_table: TableName,
        expected_insert_columns: Vec<ColumnName>,
        expected_insert_values: Vec<SqlValues>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_modification_processor_with_sql() -> ApllodbResult<()> {
        let parser = ApllodbSqlParser::default();

        static TEST_DATA: Lazy<Box<[TestDatum]>> = Lazy::new(|| {
            vec![TestDatum::new(
                "INSERT INTO people (id, age) VALUES (1, 13)",
                People::table_name(),
                vec![
                    People::sfr_id().as_column_name().clone(),
                    People::sfr_age().as_column_name().clone(),
                ],
                vec![SqlValues::new(vec![
                    SqlValue::NotNull(NNSqlValue::Integer(1)),
                    SqlValue::NotNull(NNSqlValue::Integer(13)),
                ])],
            )]
            .into_boxed_slice()
        });

        for test_datum in TEST_DATA.iter() {
            log::debug!("testing with SQL: {}", test_datum.in_insert_sql);

            // mocking insert()
            let mut engine = default_mock_engine();

            engine.expect_with_tx().returning(move || {
                let test_datum = test_datum.clone();

                let mut with_tx = MockWithTxMethods::new();
                with_tx
                    .expect_insert()
                    .with(
                        always(),
                        eq(test_datum.expected_insert_table),
                        eq(test_datum.expected_insert_columns),
                        eq(test_datum.expected_insert_values),
                    )
                    .returning(|session, _, _, _| async { Ok(session) }.boxed_local());
                with_tx
            });

            let ast = parser.parse(test_datum.in_insert_sql).unwrap();
            let session = session_with_tx(&engine).await?;
            let processor = ModificationProcessor::new(Rc::new(engine));
            processor.run(session, ast.0).await?;
        }

        Ok(())
    }
}
