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
    use apllodb_shared_components::{
        ApllodbResult, ColumnName, ColumnReference, DataType, DataTypeKind, FieldIndex, Record,
        RecordIterator, SqlValue, TableName,
    };

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
        record,
        test_support::{setup, stub_storage_engine::StubStorageEngine, utility_functions::dup},
    };

    use mockall::predicate::*;

    use super::ModificationExecutor;

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: ModificationPlanTree,
        expected_insert_records: Vec<Record>,
    }

    #[test]
    fn test_modification_executor() -> ApllodbResult<()> {
        setup();

        let (t_people, t_people_mock) = dup(TableName::new("people")?);
        let t_people_c_id = ColumnReference::new(t_people.clone(), ColumnName::new("id")?);
        let t_people_c_age = ColumnReference::new(t_people.clone(), ColumnName::new("age")?);

        let (t_people_r1, t_people_r1_mock) = dup(record! {
            FieldIndex::InColumnReference(t_people_c_id.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            FieldIndex::InColumnReference(t_people_c_age.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &13i32)?
        });
        let (t_people_r2, t_people_r2_mock) = dup(record! {
            FieldIndex::InColumnReference(t_people_c_id.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &2i32)?,
            FieldIndex::InColumnReference(t_people_c_age.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &70i32)?
        });
        let (t_people_r3, t_people_r3_mock) = dup(record! {
            FieldIndex::InColumnReference(t_people_c_id.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            FieldIndex::InColumnReference(t_people_c_age.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &35i32)?
        });

        let mut tx = StubStorageEngine::begin()?;

        // mocking
        tx.expect_insert()
            .with(
                eq(t_people.clone()),
                eq(RecordIterator::new(vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ])),
            )
            .returning(|_, _| Ok(()));

        let executor = ModificationExecutor::<'_, StubStorageEngine>::new(&tx);

        let test_data: Vec<TestDatum> = vec![
            // input from DirectInput
            TestDatum {
                in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(InsertNode {
                    table_name: t_people.clone(),
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
                expected_insert_records: vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing with input plan tree: {:#?}",
                test_datum.in_plan_tree
            );

            let modification_plan = ModificationPlan::new(test_datum.in_plan_tree.clone());
            executor.run(modification_plan)?;
        }

        Ok(())
    }
}
