mod plan_node_executor;

use apllodb_shared_components::{ApllodbResult, RecordIterator};
use apllodb_storage_engine_interface::StorageEngine;

use crate::query_plan::{plan_tree::plan_node::PlanNode, QueryPlan};

use self::plan_node_executor::PlanNodeExecutor;

/// Query executor which inputs a [QueryPlan](crate::query_plan::QueryPlan) and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(crate) struct QueryExecutor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> QueryExecutor<'exe, Engine> {
    pub(crate) fn run(&self, plan: QueryPlan) -> ApllodbResult<RecordIterator> {
        let plan_tree = plan.plan_tree;
        let root = plan_tree.root;
        self.run_dfs_post_order(root)
    }

    /// Runs `node` in post-order and returns `node`'s output.
    ///
    /// 1. Runs left child node and get output if exists.
    /// 2. Runs left child node and get output if exists.
    /// 3. Runs this `node` using inputs from left & right nodes if exist.
    /// 4. Returns `node`'s output.
    fn run_dfs_post_order(&self, node: PlanNode) -> ApllodbResult<RecordIterator> {
        let node_executor = PlanNodeExecutor::<'_, Engine>::new(self.tx);

        match node {
            PlanNode::Leaf(node_leaf) => node_executor.run_leaf(node_leaf.op),
            PlanNode::Unary(node_unary) => {
                let left_input = self.run_dfs_post_order(*node_unary.left)?;
                node_executor.run_unary(node_unary.op, left_input)
            }
            PlanNode::Binary(node_binary) => {
                let left_input = self.run_dfs_post_order(*node_binary.left)?;
                let right_input = self.run_dfs_post_order(*node_binary.right)?;
                node_executor.run_binary(node_binary.op, left_input, right_input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use apllodb_shared_components::{
        ApllodbResult, ColumnName, DataType, DataTypeKind, FieldIndex, Record, SqlValue, TableName,
    };
    use apllodb_storage_engine_interface::ProjectionQuery;

    use crate::{
        query_plan::{
            plan_tree::{
                plan_node::{
                    BinaryPlanOperation, LeafPlanOperation, PlanNode, PlanNodeBinary, PlanNodeLeaf,
                    PlanNodeUnary, UnaryPlanOperation,
                },
                PlanTree,
            },
            QueryPlan,
        },
        record,
        test_support::{
            setup,
            stub_storage_engine::{
                stub_data::StubTable, StubData, StubRowIterator, StubStorageEngine,
            },
        },
    };

    use super::QueryExecutor;

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: PlanTree,
        expected_records: Vec<Record>,
    }

    fn projection(r: Record, fields: Vec<&str>) -> ApllodbResult<Record> {
        r.projection(&fields.into_iter().map(FieldIndex::from).collect())
    }

    #[test]
    fn test_query_executor() -> ApllodbResult<()> {
        setup();

        let t_people = TableName::new("people")?;
        let t_body = TableName::new("body")?;
        let t_pet = TableName::new("pet")?;

        let t_people_r1 = record! {
            "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &13i32)?
        };
        let t_people_r2 = record! {
            "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &2i32)?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &70i32)?
        };
        let t_people_r3 = record! {
            "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &35i32)?
        };

        let t_body_r1 = record! {
            "people_id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            "height" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &145i32)?
        };
        let t_body_r3 = record! {
            "people_id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            "height" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &175i32)?
        };

        let t_pet_r1 = record! {
            "people_id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            "kind" => SqlValue::pack(&DataType::new(DataTypeKind::Text, false), &"dog".to_string())?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &13i16)?
        };
        let t_pet_r3_1 = record! {
            "people_id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            "kind" => SqlValue::pack(&DataType::new(DataTypeKind::Text, false), &"dog".to_string())?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &5i16)?
        };
        let t_pet_r3_2 = record! {
            "people_id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            "kind" => SqlValue::pack(&DataType::new(DataTypeKind::Text, false), &"cat".to_string())?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &3i16)?
        };

        let tx = StubStorageEngine::begin_stub_tx(StubData::new(vec![
            StubTable::new(
                t_people.clone(),
                StubRowIterator::from(vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ]),
            ),
            StubTable::new(
                t_body.clone(),
                StubRowIterator::from(vec![t_body_r1.clone(), t_body_r3.clone()]),
            ),
            StubTable::new(
                t_pet.clone(),
                StubRowIterator::from(vec![
                    t_pet_r1.clone(),
                    t_pet_r3_1.clone(),
                    t_pet_r3_2.clone(),
                ]),
            ),
        ]))?;
        let executor = QueryExecutor::<'_, StubStorageEngine>::new(&tx);

        let test_data: Vec<TestDatum> = vec![
            // SeqScan (with storage engine layer projection)
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Leaf(PlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: t_people.clone(),
                        projection: ProjectionQuery::All,
                    },
                })),
                expected_records: vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
            },
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Leaf(PlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: t_people.clone(),
                        projection: ProjectionQuery::ColumnNames(vec![ColumnName::new("id")?]),
                    },
                })),
                expected_records: vec![
                    projection(t_people_r1.clone(), vec!["id"])?,
                    projection(t_people_r2.clone(), vec!["id"])?,
                    projection(t_people_r3.clone(), vec!["id"])?,
                ],
            },
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Leaf(PlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: t_people.clone(),
                        projection: ProjectionQuery::ColumnNames(vec![ColumnName::new("age")?]),
                    },
                })),
                expected_records: vec![
                    projection(t_people_r1.clone(), vec!["age"])?,
                    projection(t_people_r2.clone(), vec!["age"])?,
                    projection(t_people_r3.clone(), vec!["age"])?,
                ],
            },
            // Projection
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Unary(PlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from("id")].into_iter().collect(),
                    },
                    left: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_people.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    projection(t_people_r1.clone(), vec!["id"])?,
                    projection(t_people_r2.clone(), vec!["id"])?,
                    projection(t_people_r3.clone(), vec!["id"])?,
                ],
            },
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Unary(PlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from("age")].into_iter().collect(),
                    },
                    left: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_people.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    projection(t_people_r1.clone(), vec!["age"])?,
                    projection(t_people_r2.clone(), vec!["age"])?,
                    projection(t_people_r3.clone(), vec!["age"])?,
                ],
            },
            // HashJoin
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Binary(PlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from("id"),
                        right_field: FieldIndex::from("people_id"),
                    },
                    left: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_people.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_body.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    t_people_r1.clone().join(t_body_r1.clone())?,
                    t_people_r3.clone().join(t_body_r3.clone())?,
                ],
            },
            TestDatum {
                // right has 2 same join keys
                in_plan_tree: PlanTree::new(PlanNode::Binary(PlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from("id"),
                        right_field: FieldIndex::from("people_id"),
                    },
                    left: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_people.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_pet.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    t_people_r2.clone().join(t_pet_r1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_2.clone())?,
                ],
            },
            TestDatum {
                // left has 2 same join keys
                in_plan_tree: PlanTree::new(PlanNode::Binary(PlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from("people_id"),
                        right_field: FieldIndex::from("id"),
                    },
                    left: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_pet.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: t_people.clone(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    t_people_r2.clone().join(t_pet_r1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_2.clone())?,
                ],
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing with input plan tree: {:#?}",
                test_datum.in_plan_tree
            );

            let query_plan = QueryPlan::new(test_datum.in_plan_tree.clone());
            let result = executor.run(query_plan)?;

            assert_eq!(result.collect::<Vec<Record>>(), test_datum.expected_records,);
        }
        Ok(())
    }
}
