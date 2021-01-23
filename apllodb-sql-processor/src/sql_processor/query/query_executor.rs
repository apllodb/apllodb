mod plan_node_executor;

use std::rc::Rc;

use apllodb_shared_components::{ApllodbResult, RecordIterator, SessionWithTx};
use apllodb_storage_engine_interface::StorageEngine;

use self::plan_node_executor::PlanNodeExecutor;
use crate::sql_processor::query::query_plan::{
    query_plan_tree::query_plan_node::QueryPlanNode, QueryPlan,
};
use async_recursion::async_recursion;

/// Query executor which inputs a QueryPlan and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(crate) struct QueryExecutor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> QueryExecutor<Engine> {
    pub(crate) async fn run(
        &self,
        session: SessionWithTx,
        plan: QueryPlan,
    ) -> ApllodbResult<(RecordIterator, SessionWithTx)> {
        let plan_tree = plan.plan_tree;
        let root = plan_tree.root;
        self.run_dfs_post_order(session, root).await
    }

    /// Runs `node` in post-order and returns `node`'s output.
    ///
    /// 1. Runs left child node and get output if exists.
    /// 2. Runs left child node and get output if exists.
    /// 3. Runs this `node` using inputs from left & right nodes if exist.
    /// 4. Returns `node`'s output.
    #[async_recursion(?Send)]
    async fn run_dfs_post_order(
        &self,
        session: SessionWithTx,
        node: QueryPlanNode,
    ) -> ApllodbResult<(RecordIterator, SessionWithTx)> {
        let executor = PlanNodeExecutor::new(self.engine.clone());

        match node {
            QueryPlanNode::Leaf(node_leaf) => executor.run_leaf(session, node_leaf.op).await,
            QueryPlanNode::Unary(node_unary) => {
                let (left_input, session) =
                    self.run_dfs_post_order(session, *node_unary.left).await?;
                let records = executor.run_unary(node_unary.op, left_input)?;
                Ok((records, session))
            }
            QueryPlanNode::Binary(node_binary) => {
                let (left_input, session) =
                    self.run_dfs_post_order(session, *node_binary.left).await?;
                let (right_input, session) =
                    self.run_dfs_post_order(session, *node_binary.right).await?;
                let records = executor.run_binary(node_binary.op, left_input, right_input)?;
                Ok((records, session))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use apllodb_shared_components::{ApllodbResult, FieldIndex, Record};
    use apllodb_storage_engine_interface::{
        test_support::{
            default_mock_engine, mock_select_with_models, session_with_tx,
            test_models::{Body, People, Pet},
            ModelsMock,
        },
        ProjectionQuery,
    };

    use crate::{
        sql_processor::query::query_plan::{
            query_plan_tree::{
                query_plan_node::{
                    BinaryPlanOperation, LeafPlanOperation, QueryPlanNode, QueryPlanNodeBinary,
                    QueryPlanNodeLeaf, QueryPlanNodeUnary, UnaryPlanOperation,
                },
                QueryPlanTree,
            },
            QueryPlan,
        },
        test_support::{setup, utility_functions::r_projection},
    };

    use super::QueryExecutor;

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_plan_tree: QueryPlanTree,
        expected_select_records: Vec<Record>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_query_executor() -> ApllodbResult<()> {
        setup();

        let t_people_r1 = People::record(1, 13);
        let t_people_r2 = People::record(2, 70);
        let t_people_r3 = People::record(3, 35);

        let t_body_r1 = Body::record(1, 145);
        let t_body_r3 = Body::record(3, 175);

        let t_pet_r1 = Pet::record(1, "dog", 13);
        let t_pet_r3_1 = Pet::record(3, "dog", 5);
        let t_pet_r3_2 = Pet::record(3, "cat", 3);

        let test_data: Vec<TestDatum> = vec![
            // SeqScan (with storage engine layer projection)
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: ProjectionQuery::All,
                    },
                })),
                expected_select_records: vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
            },
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: ProjectionQuery::ColumnNames(vec![People::colref_id()
                            .as_column_name()
                            .clone()]),
                    },
                })),
                expected_select_records: vec![
                    r_projection(t_people_r1.clone(), vec![People::colref_id()])?,
                    r_projection(t_people_r2.clone(), vec![People::colref_id()])?,
                    r_projection(t_people_r3.clone(), vec![People::colref_id()])?,
                ],
            },
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: ProjectionQuery::ColumnNames(vec![People::colref_age()
                            .as_column_name()
                            .clone()]),
                    },
                })),
                expected_select_records: vec![
                    r_projection(t_people_r1.clone(), vec![People::colref_age()])?,
                    r_projection(t_people_r2.clone(), vec![People::colref_age()])?,
                    r_projection(t_people_r3.clone(), vec![People::colref_age()])?,
                ],
            },
            // Projection
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::InColumnReference(People::colref_id())]
                            .into_iter()
                            .collect(),
                    },
                    left: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_select_records: vec![
                    r_projection(t_people_r1.clone(), vec![People::colref_id()])?,
                    r_projection(t_people_r2.clone(), vec![People::colref_id()])?,
                    r_projection(t_people_r3.clone(), vec![People::colref_id()])?,
                ],
            },
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::InColumnReference(People::colref_age())]
                            .into_iter()
                            .collect(),
                    },
                    left: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_select_records: vec![
                    r_projection(t_people_r1.clone(), vec![People::colref_age()])?,
                    r_projection(t_people_r2.clone(), vec![People::colref_age()])?,
                    r_projection(t_people_r3.clone(), vec![People::colref_age()])?,
                ],
            },
            // HashJoin
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::InColumnReference(People::colref_id()),
                        right_field: FieldIndex::InColumnReference(Body::colref_people_id()),
                    },
                    left: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Body::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_select_records: vec![
                    t_people_r1.clone().join(t_body_r1.clone())?,
                    t_people_r3.clone().join(t_body_r3.clone())?,
                ],
            },
            TestDatum {
                // right has 2 same join keys
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::InColumnReference(People::colref_id()),
                        right_field: FieldIndex::InColumnReference(Pet::colref_people_id()),
                    },
                    left: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_select_records: vec![
                    t_people_r1.clone().join(t_pet_r1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_2.clone())?,
                ],
            },
            TestDatum {
                // left has 2 same join keys
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::InColumnReference(Pet::colref_people_id()),
                        right_field: FieldIndex::InColumnReference(People::colref_id()),
                    },
                    left: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_select_records: vec![
                    t_people_r1.clone().join(t_pet_r1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_1.clone())?,
                    t_people_r3.clone().join(t_pet_r3_2.clone())?,
                ],
            },
            TestDatum {
                // Eq comparison with Integer & SmallInt
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::InColumnReference(People::colref_age()),
                        right_field: FieldIndex::InColumnReference(Pet::colref_age()),
                    },
                    left: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                    right: Box::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: ProjectionQuery::All,
                        },
                    })),
                })),
                expected_select_records: vec![t_people_r1.clone().join(t_pet_r1.clone())?],
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing with input plan tree: {:#?}",
                test_datum.in_plan_tree
            );

            // mocking select()
            let mut engine = default_mock_engine();
            mock_select_with_models(
                &mut engine,
                ModelsMock {
                    people: vec![
                        t_people_r1.clone(),
                        t_people_r2.clone(),
                        t_people_r3.clone(),
                    ],
                    body: vec![t_body_r1.clone(), t_body_r3.clone()],
                    pet: vec![t_pet_r1.clone(), t_pet_r3_1.clone(), t_pet_r3_2.clone()],
                },
            );

            let session = session_with_tx(&engine).await?;
            let executor = QueryExecutor::new(Rc::new(engine));
            let query_plan = QueryPlan::new(test_datum.in_plan_tree.clone());
            let (result, _) = executor.run(session, query_plan).await?;

            assert_eq!(
                result.collect::<Vec<Record>>(),
                test_datum.expected_select_records,
            );
        }
        Ok(())
    }
}