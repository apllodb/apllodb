use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, ApllodbSessionError, ApllodbSessionResult, AstTranslator,
    ColumnName, CorrelationReference, FieldReference, FullFieldReference, Record,
    RecordFieldRefSchema, Records, Session, SessionWithTx, SqlValue, SqlValues,
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

use super::sql_processor_context::SQLProcessorContext;

pub(crate) mod modification_executor;
pub(crate) mod modification_plan;

/// Processes ÎNSERT/UPDATE/DELETE command.
#[derive(Debug, new)]
pub(crate) struct ModificationProcessor<Engine: StorageEngine> {
    context: Arc<SQLProcessorContext<Engine>>,
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
                    let executor = ModificationExecutor::new(self.context.clone());
                    executor.run(session, plan).await
                }
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            },
            _ => Err(ApllodbSessionError::new(
                ApllodbError::feature_not_supported("only INSERT is supported for DML currently"),
                Session::from(session),
            )),
        }
    }

    fn run_helper_insert(&self, command: InsertCommand) -> ApllodbResult<ModificationPlan> {
        if command.alias.is_some() {
            ApllodbError::feature_not_supported(
                "table alias in INSERT command is not currently supported",
            );
        }

        let ast_table_name = command.table_name;
        let table_name = AstTranslator::table_name(ast_table_name.clone())?;

        let ast_column_names = command.column_names.into_vec();
        let column_names: Vec<ColumnName> = ast_column_names
            .into_iter()
            .map(AstTranslator::column_name)
            .collect::<ApllodbResult<_>>()?;
        let column_names_len = column_names.len();
        let insert_values = command.values.into_vec();

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

        let records: Vec<Record> = insert_values
            .into_iter()
            .map(|insert_value| {
                let expressions = insert_value.expressions.into_vec();

                if column_names_len != expressions.len() {
                    ApllodbError::feature_not_supported(
                        "VALUES expressions and column names must have same length currently",
                    );
                }

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

                let values = SqlValues::new(constant_values);
                Ok(Record::new(values))
            })
            .collect::<ApllodbResult<_>>()?;

        let records_query_node_id =
            self.context
                .node_repo
                .create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::Values {
                        records: Records::new(schema, records),
                    },
                }));

        let plan_node = ModificationPlanNode::Insert(InsertNode {
            table_name,
            child: records_query_node_id,
        });

        Ok(ModificationPlan::new(ModificationPlanTree::new(plan_node)))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::sql_processor::sql_processor_context::SQLProcessorContext;
    use apllodb_shared_components::{
        test_support::test_models::People, ApllodbResult, ColumnName, NNSqlValue, SqlValue,
        SqlValues, TableName,
    };
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::test_support::{default_mock_engine, MockWithTxMethods};
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

            let context = Arc::new(SQLProcessorContext::new(engine));

            let ast = parser.parse(test_datum.in_insert_sql).unwrap();
            ModificationProcessor::run_directly(context.clone(), ast.0).await?;
        }

        Ok(())
    }
}
