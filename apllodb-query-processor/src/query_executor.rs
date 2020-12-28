mod plan_node_executor;

use apllodb_shared_components::{ApllodbResult, RecordIterator};
use apllodb_storage_engine_interface::StorageEngine;

use crate::query_plan::{
    plan_tree::plan_node::{PlanNode, PlanNodeBinary},
    QueryPlan,
};

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
            PlanNode::Binary(PlanNodeBinary { op, left, right }) => {
                let left_input = self.run_dfs_post_order(*left)?;
                let right_input = self.run_dfs_post_order(*right)?;
                todo!()
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
                    LeafPlanOperation, PlanNode, PlanNodeLeaf, PlanNodeUnary, UnaryPlanOperation,
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

        let r1 = record! {
            "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &13i32)?
        };
        let r2 = record! {
            "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &2i32)?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &70i32)?
        };
        let r3 = record! {
            "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
            "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &35i32)?
        };

        let tx = StubStorageEngine::begin_stub_tx(StubData::new(vec![StubTable::new(
            TableName::new("t")?,
            StubRowIterator::from(vec![r1.clone(), r2.clone(), r3.clone()]),
        )]))?;
        let executor = QueryExecutor::<'_, StubStorageEngine>::new(&tx);

        let test_data: Vec<TestDatum> = vec![
            // SeqScan (with storage engine layer projection)
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Leaf(PlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: TableName::new("t")?,
                        projection: ProjectionQuery::All,
                    },
                })),
                expected_records: vec![r1.clone(), r2.clone(), r3.clone()],
            },
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Leaf(PlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: TableName::new("t")?,
                        projection: ProjectionQuery::ColumnNames(vec![ColumnName::new("id")?]),
                    },
                })),
                expected_records: vec![
                    projection(r1.clone(), vec!["id"])?,
                    projection(r2.clone(), vec!["id"])?,
                    projection(r3.clone(), vec!["id"])?,
                ],
            },
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Leaf(PlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: TableName::new("t")?,
                        projection: ProjectionQuery::ColumnNames(vec![ColumnName::new("age")?]),
                    },
                })),
                expected_records: vec![
                    projection(r1.clone(), vec!["age"])?,
                    projection(r2.clone(), vec!["age"])?,
                    projection(r3.clone(), vec!["age"])?,
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
                            table_name: TableName::new("t")?,
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    projection(r1.clone(), vec!["id"])?,
                    projection(r2.clone(), vec!["id"])?,
                    projection(r3.clone(), vec!["id"])?,
                ],
            },
            TestDatum {
                in_plan_tree: PlanTree::new(PlanNode::Unary(PlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from("age")].into_iter().collect(),
                    },
                    left: Box::new(PlanNode::Leaf(PlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: TableName::new("t")?,
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_records: vec![
                    projection(r1.clone(), vec!["age"])?,
                    projection(r2.clone(), vec!["age"])?,
                    projection(r3.clone(), vec!["age"])?,
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
