mod plan_node_executor;

use std::rc::Rc;

use apllodb_shared_components::{
    ApllodbSessionError, ApllodbSessionResult, Records, Session, SessionWithTx,
};
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
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
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
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        let executor = PlanNodeExecutor::new(self.engine.clone());

        match node {
            QueryPlanNode::Leaf(node_leaf) => executor.run_leaf(session, node_leaf.op).await,
            QueryPlanNode::Unary(node_unary) => {
                let (left_input, session) =
                    self.run_dfs_post_order(session, *node_unary.left).await?;
                match executor.run_unary(node_unary.op, left_input) {
                    Ok(records) => Ok((records, session)),
                    Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
                }
            }
            QueryPlanNode::Binary(node_binary) => {
                let (left_input, session) =
                    self.run_dfs_post_order(session, *node_binary.left).await?;
                let (right_input, session) =
                    self.run_dfs_post_order(session, *node_binary.right).await?;
                match executor.run_binary(node_binary.op, left_input, right_input) {
                    Ok(records) => Ok((records, session)),
                    Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use pretty_assertions::assert_eq;

    use apllodb_shared_components::{
        test_support::{
            fixture::*,
            test_models::{Body, People, Pet},
        },
        ApllodbResult, FieldIndex, Record, RecordFieldRefSchema,
    };
    use apllodb_storage_engine_interface::{
        test_support::{default_mock_engine, mock_select, session_with_tx, MockWithTxMethods},
        ProjectionQuery,
    };

    use crate::sql_processor::query::query_plan::{
        query_plan_tree::{
            query_plan_node::{
                BinaryPlanOperation, LeafPlanOperation, QueryPlanNode, QueryPlanNodeBinary,
                QueryPlanNodeLeaf, QueryPlanNodeUnary, UnaryPlanOperation,
            },
            QueryPlanTree,
        },
        QueryPlan,
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
        let mut engine = default_mock_engine();
        engine.expect_with_tx().returning(|| {
            let mut with_tx = MockWithTxMethods::new();
            // mocking select()
            mock_select(&mut with_tx, &FULL_MODELS);
            with_tx
        });
        let engine = Rc::new(engine);

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
                    PEOPLE_RECORD1.clone(),
                    PEOPLE_RECORD2.clone(),
                    PEOPLE_RECORD3.clone(),
                ],
            },
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: ProjectionQuery::Schema(RecordFieldRefSchema::factory(vec![
                            People::ffr_id(),
                        ])),
                    },
                })),
                expected_select_records: vec![
                    PEOPLE_RECORD1
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                    PEOPLE_RECORD2
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                    PEOPLE_RECORD3
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                ],
            },
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: ProjectionQuery::Schema(RecordFieldRefSchema::factory(vec![
                            People::ffr_age(),
                        ])),
                    },
                })),
                expected_select_records: vec![
                    PEOPLE_RECORD1
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                    PEOPLE_RECORD2
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                    PEOPLE_RECORD3
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                ],
            },
            // Projection
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from(People::ffr_id())]
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
                    PEOPLE_RECORD1
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                    PEOPLE_RECORD2
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                    PEOPLE_RECORD3
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                ],
            },
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from(People::ffr_age())]
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
                    PEOPLE_RECORD1
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                    PEOPLE_RECORD2
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                    PEOPLE_RECORD3
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                ],
            },
            // HashJoin
            TestDatum {
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from(People::ffr_id()),
                        right_field: FieldIndex::from(Body::ffr_people_id()),
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
                    PEOPLE_RECORD1.clone().join(BODY_RECORD1.clone()),
                    PEOPLE_RECORD3.clone().join(BODY_RECORD3.clone()),
                ],
            },
            TestDatum {
                // right has 2 same join keys
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from(People::ffr_id()),
                        right_field: FieldIndex::from(Pet::ffr_people_id()),
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
                    PEOPLE_RECORD1.clone().join(PET_RECORD1.clone()),
                    PEOPLE_RECORD3.clone().join(PET_RECORD3_1.clone()),
                    PEOPLE_RECORD3.clone().join(PET_RECORD3_2.clone()),
                ],
            },
            TestDatum {
                // left has 2 same join keys
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from(Pet::ffr_people_id()),
                        right_field: FieldIndex::from(People::ffr_id()),
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
                    PET_RECORD1.clone().join(PEOPLE_RECORD1.clone()),
                    PET_RECORD3_1.clone().join(PEOPLE_RECORD3.clone()),
                    PET_RECORD3_2.clone().join(PEOPLE_RECORD3.clone()),
                ],
            },
            TestDatum {
                // Eq comparison with Integer & SmallInt
                in_plan_tree: QueryPlanTree::new(QueryPlanNode::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        left_field: FieldIndex::from(People::ffr_age()),
                        right_field: FieldIndex::from(Pet::ffr_age()),
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
                expected_select_records: vec![PEOPLE_RECORD1.clone().join(PET_RECORD1.clone())],
            },
        ];

        for test_datum in test_data {
            log::debug!(
                "testing with input plan tree: {:#?}",
                test_datum.in_plan_tree
            );

            let session = session_with_tx(engine.as_ref()).await?;
            let executor = QueryExecutor::new(engine.clone());
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
