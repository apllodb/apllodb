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
    use std::collections::HashSet;

    use apllodb_shared_components::{
        ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, ColumnReference, DataType,
        DataTypeKind, FieldIndex, Record, RecordIterator, SqlValue, TableName,
    };
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
        record,
        test_support::{setup, stub_storage_engine::StubStorageEngine, utility_functions::dup},
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

        let (t_people, t_people_mock) = dup(TableName::new("people")?);
        let t_people_c_id = ColumnReference::new(t_people.clone(), ColumnName::new("id")?);
        let t_people_c_age = ColumnReference::new(t_people.clone(), ColumnName::new("age")?);

        let (t_pet, t_pet_mock) = dup(TableName::new("pet")?);
        let t_pet_c_people_id = ColumnReference::new(t_pet.clone(), ColumnName::new("people_id")?);
        let t_pet_c_kind = ColumnReference::new(t_pet.clone(), ColumnName::new("kind")?);
        let t_pet_c_age = ColumnReference::new(t_pet.clone(), ColumnName::new("age")?);

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

        let (t_pet_r1, t_pet_r1_mock) = dup(record! {
            FieldIndex::InColumnReference(t_pet_c_people_id.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            FieldIndex::InColumnReference(t_pet_c_kind.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Text, false), &"dog".to_string())?,
            FieldIndex::InColumnReference(t_pet_c_age.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &13i16)?
        });
        let (t_pet_r3_1, t_pet_r3_1_mock) = dup(record! {
            FieldIndex::InColumnReference(t_pet_c_people_id.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            FieldIndex::InColumnReference(t_pet_c_kind.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Text, false), &"dog".to_string())?,
            FieldIndex::InColumnReference(t_pet_c_age.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &5i16)?
        });
        let (t_pet_r3_2, t_pet_r3_2_mock) = dup(record! {
            FieldIndex::InColumnReference(t_pet_c_people_id.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            FieldIndex::InColumnReference(t_pet_c_kind.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::Text, false), &"cat".to_string())?,
            FieldIndex::InColumnReference(t_pet_c_age.clone()) => SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &3i16)?
        });

        let mut tx = StubStorageEngine::begin()?;

        // mocking select()
        tx.expect_select().returning(move |table_name, projection| {
            let records: Vec<Record> = if *table_name == t_pet_mock {
                vec![
                    t_pet_r1_mock.clone(),
                    t_pet_r3_1_mock.clone(),
                    t_pet_r3_2_mock.clone(),
                ]
            } else {
                return Err(ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!("table `{:?}` is undefined in StubTx", table_name),
                    None,
                ));
            };

            let projected_records: Vec<Record> = match projection {
                ProjectionQuery::All => records,
                ProjectionQuery::ColumnNames(column_names) => {
                    let fields: HashSet<FieldIndex> = column_names
                        .into_iter()
                        .map(|cn| {
                            FieldIndex::InColumnReference(ColumnReference::new(
                                table_name.clone(),
                                cn,
                            ))
                        })
                        .collect();

                    records
                        .into_iter()
                        .map(|record| Ok(record.projection(&fields)?))
                        .collect::<ApllodbResult<_>>()?
                }
            };

            Ok(RecordIterator::new(projected_records))
        });

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
                expected_insert_table: t_people.clone(),
                expected_insert_records: vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
            },
            // input from same table records (dup)
            TestDatum {
                in_plan_tree: ModificationPlanTree::new(ModificationPlanNode::Insert(InsertNode {
                    table_name: t_pet.clone(),
                    child: QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_pet.clone(),
                            projection: ProjectionQuery::All,
                        },
                    }),
                })),
                expected_insert_table: t_pet.clone(),
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

            let executor = ModificationExecutor::<'_, StubStorageEngine>::new(&tx);
            executor.run(modification_plan)?;
        }

        Ok(())
    }
}
