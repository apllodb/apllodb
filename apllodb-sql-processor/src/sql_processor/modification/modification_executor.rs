use std::rc::Rc;

use apllodb_shared_components::{ApllodbSessionResult, SessionWithTx};
use apllodb_storage_engine_interface::{StorageEngine, WithTxMethods};

use crate::sql_processor::query::{
    query_executor::QueryExecutor,
    query_plan::{query_plan_tree::QueryPlanTree, QueryPlan},
};

use super::modification_plan::{
    modification_plan_tree::modification_plan_node::ModificationPlanNode, ModificationPlan,
};

/// Modification (INSERT, UPDATE, and DELETE) executor which inputs a ModificationPlan requests to storage engine.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct ModificationExecutor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> ModificationExecutor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

    pub(crate) async fn run(
        &self,
        session: SessionWithTx,
        plan: ModificationPlan,
    ) -> ApllodbSessionResult<SessionWithTx> {
        let query_executor = QueryExecutor::new(self.engine.clone());
        let plan_tree = plan.plan_tree;
        match plan_tree.root {
            ModificationPlanNode::Insert(insert_node) => {
                let input_query_plan_root = insert_node.child;
                let (input, session) = query_executor
                    .run(
                        session,
                        QueryPlan::new(QueryPlanTree::new(input_query_plan_root)),
                    )
                    .await?;

                let session = self
                    .engine
                    .with_tx()
                    .insert(
                        session,
                        insert_node.table_name,
                        input
                            .as_full_field_references()
                            .iter()
                            .map(|ffr| ffr.as_column_name())
                            .cloned()
                            .collect(),
                        input.into_sql_values(),
                    )
                    .await?;

                Ok(session)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use apllodb_shared_components::{
        test_support::{
            fixture::*,
            test_models::{People, Pet},
        },
        ApllodbResult, ColumnName, Record, Records, SqlValues, TableName,
    };
    use apllodb_storage_engine_interface::{
        test_support::{default_mock_engine, mock_select, session_with_tx, MockWithTxMethods},
        AliasDef, ProjectionQuery,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    use crate::sql_processor::{
        modification::modification_plan::{
            modification_plan_tree::{
                modification_plan_node::{InsertNode, ModificationPlanNode},
                ModificationPlanTree,
            },
            ModificationPlan,
        },
        query::query_plan::query_plan_tree::query_plan_node::{
            LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf,
        },
    };

    use super::ModificationExecutor;

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: ModificationPlanTree,
        expected_insert_table: TableName,
        expected_insert_columns: Vec<ColumnName>,
        expected_insert_records: Vec<Record>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_modification_executor() -> ApllodbResult<()> {
        static TEST_DATA: Lazy<Box<[TestDatum]>> = Lazy::new(|| {
            vec![
                // input from DirectInput
                TestDatum {
                    in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(
                        InsertNode {
                            table_name: People::table_name(),
                            child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                                op: LeafPlanOperation::Values {
                                    records: Records::new(
                                        People::schema(),
                                        vec![
                                            PEOPLE_RECORD1.clone(),
                                            PEOPLE_RECORD2.clone(),
                                            PEOPLE_RECORD3.clone(),
                                        ],
                                    ),
                                },
                            }),
                        },
                    )),
                    expected_insert_table: People::table_name(),
                    expected_insert_columns: People::schema().to_column_names(),
                    expected_insert_records: vec![
                        PEOPLE_RECORD1.clone(),
                        PEOPLE_RECORD2.clone(),
                        PEOPLE_RECORD3.clone(),
                    ],
                },
                // input from same table records (dup)
                TestDatum {
                    in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(
                        InsertNode {
                            table_name: Pet::table_name(),
                            child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                                op: LeafPlanOperation::SeqScan {
                                    table_name: Pet::table_name(),
                                    projection: ProjectionQuery::All,
                                    alias_def: AliasDef::default(),
                                },
                            }),
                        },
                    )),
                    expected_insert_table: Pet::table_name(),
                    expected_insert_columns: Pet::schema().to_column_names(),
                    expected_insert_records: vec![
                        PET_RECORD1.clone(),
                        PET_RECORD3_1.clone(),
                        PET_RECORD3_2.clone(),
                    ],
                },
            ]
            .into_boxed_slice()
        });

        for test_datum in TEST_DATA.iter() {
            log::debug!(
                "testing with input plan tree: {:#?}",
                test_datum.in_plan_tree
            );

            let modification_plan = ModificationPlan::new(test_datum.in_plan_tree.clone());

            let mut engine = default_mock_engine();
            engine.expect_with_tx().returning(move || {
                let test_datum = test_datum.clone();

                let mut with_tx = MockWithTxMethods::new();

                // mocking select()
                mock_select(&mut with_tx, &PET_MODELS);

                // mocking insert()
                with_tx
                    .expect_insert()
                    .with(
                        always(),
                        eq(test_datum.expected_insert_table),
                        eq(test_datum.expected_insert_columns),
                        eq(test_datum
                            .expected_insert_records
                            .into_iter()
                            .map(|r| r.into_values())
                            .collect::<Vec<SqlValues>>()),
                    )
                    .returning(|session, _, _, _| async { Ok(session) }.boxed_local());

                with_tx
            });
            let engine = Rc::new(engine);

            let session = session_with_tx(engine.as_ref()).await?;
            let executor = ModificationExecutor::new(engine.clone());
            executor.run(session, modification_plan).await?;
        }

        Ok(())
    }
}
