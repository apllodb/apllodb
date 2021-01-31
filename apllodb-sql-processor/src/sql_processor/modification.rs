use std::{collections::HashMap, convert::TryFrom, rc::Rc};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, ColumnReference, FieldIndex, Record,
    RecordIterator, Session, SessionWithTx, SqlValue,
};
use apllodb_sql_parser::apllodb_ast::{Command, InsertCommand};
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    ast_translator::AstTranslator,
    sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
        LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf,
    },
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

        let table_name = AstTranslator::table_name(command.table_name)?;

        let column_names = command.column_names.into_vec();
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

        let fields: HashMap<FieldIndex, SqlValue> = column_names
            .into_iter()
            .zip(constant_values)
            .into_iter()
            .map(|(cn, sql_value)| {
                let col_ref =
                    ColumnReference::new(table_name.clone(), AstTranslator::column_name(cn)?);
                let field = FieldIndex::InColumnReference(col_ref);
                Ok((field, sql_value))
            })
            .collect::<ApllodbResult<_>>()?;

        let record = Record::new(fields);
        let records = RecordIterator::new(vec![record]);

        let plan_node = ModificationPlanNode::Insert(InsertNode {
            table_name,
            child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                op: LeafPlanOperation::DirectInput { records },
            }),
        });

        Ok(ModificationPlan::new(ModificationPlanTree::new(plan_node)))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use apllodb_shared_components::{ApllodbResult, Record, RecordIterator, TableName};
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, session_with_tx, test_models::People, MockWithTxMethods,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    use super::ModificationProcessor;

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum {
        in_insert_sql: &'static str,
        expected_insert_table: TableName,
        expected_insert_records: Vec<Record>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_modification_processor_with_sql() -> ApllodbResult<()> {
        let parser = ApllodbSqlParser::default();

        static TEST_DATA: Lazy<Box<[TestDatum]>> = Lazy::new(|| {
            vec![TestDatum::new(
                "INSERT INTO people (id, age) VALUES (1, 13)",
                People::table_name(),
                vec![People::record(1, 13)],
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
                        eq(RecordIterator::new(test_datum.expected_insert_records)),
                    )
                    .returning(|session, _, _| async { Ok(session) }.boxed_local());
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
