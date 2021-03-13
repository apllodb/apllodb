use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, AstTranslator, ColumnName,
    CorrelationReference, FieldReference, FullFieldReference, Record, RecordFieldRefSchema,
    Records, Session, SessionWithTx, SqlValue, SqlValues,
};
use apllodb_sql_parser::apllodb_ast::{Command, InsertCommand};
use apllodb_storage_engine_interface::StorageEngine;

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
    node_kind::{QueryPlanNodeKind, QueryPlanNodeLeaf},
    operation::LeafPlanOperation,
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

use super::query::query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository;

pub(crate) mod modification_executor;
pub(crate) mod modification_plan;

/// Processes ÎNSERT/UPDATE/DELETE command.
#[derive(Debug, new)]
pub(crate) struct ModificationProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
    node_repo: Arc<RwLock<QueryPlanNodeRepository>>,
}

impl<Engine: StorageEngine> ModificationProcessor<Engine> {}

impl<Engine: StorageEngine> ModificationProcessor<Engine> {
    /// Executes parsed INSERT/UPDATE/DELETE command.
    pub async fn run(
        &self,
        session: SessionWithTx,
        command: Command,
    ) -> ApllodbSessionResult<SessionWithTx> {
        match command {
            Command::InsertCommandVariant(ic) => match self.run_helper_insert(ic) {
                Ok(plan) => {
                    let executor =
                        ModificationExecutor::new(self.engine.clone(), self.node_repo.clone());
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

        let ffrs: Vec<FullFieldReference> = column_names
            .into_iter()
            .map(|cn| {
                FullFieldReference::new(
                    CorrelationReference::TableNameVariant(table_name.clone()),
                    FieldReference::ColumnNameVariant(cn),
                )
            })
            .collect();
        let schema = RecordFieldRefSchema::new(ffrs);

        let constant_values: Vec<SqlValue> = expressions
            .into_iter()
            .map(|ast_expression| {
                let expression = AstTranslator::expression_in_non_select(
                    ast_expression,
                    vec![ast_table_name.clone()],
                )?;
                expression.to_sql_value(None)
            })
            .collect::<ApllodbResult<_>>()?;

        let insert_values = SqlValues::new(constant_values);

        let records_query_node = {
            let node_id = {
                let mut repo = self.node_repo.write().unwrap();
                repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::Values {
                        records: Records::new(schema, vec![Record::new(insert_values)]),
                    },
                }))
                .id
            };
            let mut repo = self.node_repo.write().unwrap();
            repo.remove(node_id)?
        };

        let plan_node = ModificationPlanNode::Insert(InsertNode {
            table_name,
            child: records_query_node,
        });

        Ok(ModificationPlan::new(ModificationPlanTree::new(plan_node)))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        rc::Rc,
        sync::{Arc, RwLock},
    };

    use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository;
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
                    People::ffr_id().as_column_name().clone(),
                    People::ffr_age().as_column_name().clone(),
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
            let processor = ModificationProcessor::new(
                Rc::new(engine),
                Arc::new(RwLock::new(QueryPlanNodeRepository::default())),
            );
            processor.run(session, ast.0).await?;
        }

        Ok(())
    }
}
