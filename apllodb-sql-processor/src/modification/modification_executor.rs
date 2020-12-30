use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

use crate::query::{
    query_executor::QueryExecutor,
    query_plan::{query_plan_tree::QueryPlanTree, QueryPlan},
};

use super::modification_plan::{
    modification_plan_tree::modification_plan_node::ModificationPlanNode, ModificationPlan,
};

/// Modification (INSERT, UPDATE, and DELETE) executor which inputs a [ModificationPlan](crate::modification_plan::ModificationPlan) and r expected_insert_records: ()equests modification to storage engine.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(crate) struct ModificationExecutor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> ModificationExecutor<'exe, Engine> {
    pub(crate) fn run(&self, plan: ModificationPlan) -> ApllodbResult<()> {
        let plan_tree = plan.plan_tree;
        match plan_tree.root {
            ModificationPlanNode::Insert(insert_node) => {
                let query_executor = QueryExecutor::<'_, Engine>::new(self.tx);
                let input_query_plan_root = insert_node.child;
                let input = query_executor
                    .run(QueryPlan::new(QueryPlanTree::new(input_query_plan_root)))?;

                self.tx.insert(&insert_node.table_name, input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{ApllodbResult, Record, RecordIterator, TableName};
    use apllodb_storage_engine_interface::ProjectionQuery;

    use crate::{
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
        test_support::{
            mock_tx::mock_tx_select::mock_select_with_models::{
                mock_select_with_models, ModelsMock,
            },
            setup,
            test_models::{People, Pet},
            test_storage_engine::TestStorageEngine,
        },
    };

    use mockall::predicate::*;

    use super::ModificationExecutor;

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: ModificationPlanTree,
        expected_insert_table: TableName,
        expected_insert_records: Vec<Record>,
    }

    #[test]
    fn test_modification_executor() -> ApllodbResult<()> {
        setup();

        let t_people_r1 = People::record(1, 13);
        let t_people_r2 = People::record(2, 70);
        let t_people_r3 = People::record(3, 35);

        let t_pet_r1 = Pet::record(1, "dog", 13);
        let t_pet_r3_1 = Pet::record(3, "dog", 5);
        let t_pet_r3_2 = Pet::record(3, "cat", 3);

        let mut tx = TestStorageEngine::begin()?;

        mock_select_with_models(
            &mut tx,
            ModelsMock {
                pet: vec![t_pet_r1.clone(), t_pet_r3_1.clone(), t_pet_r3_2.clone()],
                ..ModelsMock::default()
            },
        );

        let test_data: Vec<TestDatum> = vec![
            // input from DirectInput
            TestDatum {
                in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(InsertNode {
                    table_name: People::table_name(),
                    child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::DirectInput {
                            records: RecordIterator::new(vec![
                                t_people_r1.clone(),
                                t_people_r2.clone(),
                                t_people_r3.clone(),
                            ]),
                        },
                    }),
                })),
                expected_insert_table: People::table_name(),
                expected_insert_records: vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
            },
            // input from same table records (dup)
            TestDatum {
                in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(InsertNode {
                    table_name: Pet::table_name(),
                    child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    }),
                })),
                expected_insert_table: Pet::table_name(),
                expected_insert_records: vec![
                    t_pet_r1.clone(),
                    t_pet_r3_1.clone(),
                    t_pet_r3_2.clone(),
                ],
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing with input plan tree: {:#?}",
                test_datum.in_plan_tree
            );

            let modification_plan = ModificationPlan::new(test_datum.in_plan_tree.clone());

            // mocking insert()
            tx.expect_insert()
                .with(
                    eq(test_datum.expected_insert_table),
                    eq(RecordIterator::new(test_datum.expected_insert_records)),
                )
                .returning(|_, _| Ok(()));

            let executor = ModificationExecutor::<'_, TestStorageEngine>::new(&tx);
            executor.run(modification_plan)?;
        }

        Ok(())
    }
}
