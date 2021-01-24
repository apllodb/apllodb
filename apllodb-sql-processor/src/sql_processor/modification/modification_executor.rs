use std::rc::Rc;

use apllodb_shared_components::{ApllodbResult, SessionWithTx};
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
    ) -> ApllodbResult<SessionWithTx> {
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
                    .insert(session, insert_node.table_name, input)
                    .await?;

                Ok(session)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use apllodb_shared_components::{ApllodbResult, Record, RecordIterator, TableName};
    use apllodb_storage_engine_interface::{
        test_support::{
            default_mock_engine,
            fixture::*,
            mock_select, session_with_tx,
            test_models::{People, Pet},
            MockWithTxMethods,
        },
        ProjectionQuery,
    };
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    use crate::{
        sql_processor::{
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
        },
        test_support::setup,
    };

    use super::ModificationExecutor;

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: ModificationPlanTree,
        expected_insert_table: TableName,
        expected_insert_records: Vec<Record>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_modification_executor() -> ApllodbResult<()> {
        setup();

        static TEST_DATA: Lazy<Box<[TestDatum]>> = Lazy::new(|| {
            vec![
                // input from DirectInput
                TestDatum {
                    in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(
                        InsertNode {
                            table_name: People::table_name(),
                            child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                                op: LeafPlanOperation::DirectInput {
                                    records: RecordIterator::new(vec![
                                        T_PEOPLE_R1.clone(),
                                        T_PEOPLE_R2.clone(),
                                        T_PEOPLE_R3.clone(),
                                    ]),
                                },
                            }),
                        },
                    )),
                    expected_insert_table: People::table_name(),
                    expected_insert_records: vec![
                        T_PEOPLE_R1.clone(),
                        T_PEOPLE_R2.clone(),
                        T_PEOPLE_R3.clone(),
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
                                },
                            }),
                        },
                    )),
                    expected_insert_table: Pet::table_name(),
                    expected_insert_records: vec![
                        T_PET_R1.clone(),
                        T_PET_R3_1.clone(),
                        T_PET_R3_2.clone(),
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
                        eq(RecordIterator::new(test_datum.expected_insert_records)),
                    )
                    .returning(|session, _, _| async { Ok(session) }.boxed_local());

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
