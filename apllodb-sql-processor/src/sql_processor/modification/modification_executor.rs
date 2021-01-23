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
    use apllodb_shared_components::{ApllodbResult, Record, TableName};

    use crate::{
        sql_processor::modification::modification_plan::modification_plan_tree::ModificationPlanTree,
        test_support::setup,
    };

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: ModificationPlanTree,
        expected_insert_table: TableName,
        expected_insert_records: Vec<Record>,
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_modification_executor() -> ApllodbResult<()> {
        setup();

        // let t_people_r1 = People::record(1, 13);
        // let t_people_r2 = People::record(2, 70);
        // let t_people_r3 = People::record(3, 35);

        // let t_pet_r1 = Pet::record(1, "dog", 13);
        // let t_pet_r3_1 = Pet::record(3, "dog", 5);
        // let t_pet_r3_2 = Pet::record(3, "cat", 3);

        // let mut dml = MockDML::new();

        // mock_select_with_models(
        //     &mut dml,
        //     ModelsMock {
        //         pet: vec![t_pet_r1.clone(), t_pet_r3_1.clone(), t_pet_r3_2.clone()],
        //         ..ModelsMock::default()
        //     },
        // );

        // let test_data: Vec<TestDatum> = vec![
        //     // input from DirectInput
        //     TestDatum {
        //         in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(InsertNode {
        //             table_name: People::table_name(),
        //             child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
        //                 op: LeafPlanOperation::DirectInput {
        //                     records: RecordIterator::new(vec![
        //                         t_people_r1.clone(),
        //                         t_people_r2.clone(),
        //                         t_people_r3.clone(),
        //                     ]),
        //                 },
        //             }),
        //         })),
        //         expected_insert_table: People::table_name(),
        //         expected_insert_records: vec![
        //             t_people_r1.clone(),
        //             t_people_r2.clone(),
        //             t_people_r3.clone(),
        //         ],
        //     },
        //     // input from same table records (dup)
        //     TestDatum {
        //         in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(InsertNode {
        //             table_name: Pet::table_name(),
        //             child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
        //                 op: LeafPlanOperation::SeqScan {
        //                     table_name: Pet::table_name(),
        //                     projection: ProjectionQuery::All,
        //                 },
        //             }),
        //         })),
        //         expected_insert_table: Pet::table_name(),
        //         expected_insert_records: vec![
        //             t_pet_r1.clone(),
        //             t_pet_r3_1.clone(),
        //             t_pet_r3_2.clone(),
        //         ],
        //     },
        // ];

        // for test_datum in test_data {
        //     log::debug!(
        //         "testing with input plan tree: {:#?}",
        //         test_datum.in_plan_tree
        //     );

        //     let modification_plan = ModificationPlan::new(test_datum.in_plan_tree.clone());

        //     let mut tx = TestTx;

        //     // mocking insert()
        //     dml.expect_insert()
        //         .with(
        //             always(),
        //             eq(test_datum.expected_insert_table),
        //             eq(RecordIterator::new(test_datum.expected_insert_records)),
        //         )
        //         .returning(|_, _, _| Ok(()));

        //     let executor = ModificationExecutor::<TestStorageEngine>::new(&dml);
        //     executor.run(&mut tx, modification_plan)?;
        // }

        Ok(())
    }
}
