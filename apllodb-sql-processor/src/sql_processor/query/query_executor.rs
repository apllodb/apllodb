mod plan_node_executor;

use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbSessionError, ApllodbSessionResult, Session, SessionWithTx,
};
use apllodb_storage_engine_interface::StorageEngine;

use self::plan_node_executor::PlanNodeExecutor;
use crate::{
    records::Records,
    sql_processor::{
        query::query_plan::{
            query_plan_tree::query_plan_node::node_kind::QueryPlanNodeKind, QueryPlan,
        },
        sql_processor_context::SqlProcessorContext,
    },
};
use async_recursion::async_recursion;

use super::query_plan::query_plan_tree::query_plan_node::node_id::QueryPlanNodeId;

/// Query executor which inputs a QueryPlan and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Debug, new)]
pub(crate) struct QueryExecutor<Engine: StorageEngine> {
    context: Arc<SqlProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> QueryExecutor<Engine> {
    pub(crate) async fn run(
        &self,
        session: SessionWithTx,
        plan: QueryPlan,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        let plan_tree = plan.plan_tree;
        self.run_dfs_post_order(session, plan_tree.root).await
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
        node_id: QueryPlanNodeId,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        let executor = PlanNodeExecutor::new(self.context.clone());

        match self.context.node_repo.remove(node_id) {
            Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            Ok(node) => match node.kind {
                QueryPlanNodeKind::Leaf(node_leaf) => {
                    executor.run_leaf(session, node_leaf.op).await
                }
                QueryPlanNodeKind::Unary(node_unary) => {
                    let (left_input, session) =
                        self.run_dfs_post_order(session, node_unary.left).await?;
                    match executor.run_unary(node_unary.op, left_input) {
                        Ok(records) => Ok((records, session)),
                        Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
                    }
                }
                QueryPlanNodeKind::Binary(node_binary) => {
                    let (left_input, session) =
                        self.run_dfs_post_order(session, node_binary.left).await?;
                    let (right_input, session) =
                        self.run_dfs_post_order(session, node_binary.right).await?;
                    match executor.run_binary(node_binary.op, left_input, right_input) {
                        Ok(records) => Ok((records, session)),
                        Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
                    }
                }
            },
        }
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use apllodb_shared_components::{
        test_support::{
            fixture::*,
            test_models::{Body, People, Pet},
        },
        ApllodbResult,
    };
    use apllodb_storage_engine_interface::{
        test_support::{default_mock_engine, mock_select, MockWithTxMethods},
        MockStorageEngine, RowProjectionQuery,
    };
    use pretty_assertions::assert_eq;

    use super::QueryExecutor;
    use crate::sql_processor::query::query_plan::{
        query_plan_tree::{
            query_plan_node::{
                node_id::QueryPlanNodeId,
                node_kind::{
                    QueryPlanNodeBinary, QueryPlanNodeKind, QueryPlanNodeLeaf, QueryPlanNodeUnary,
                },
                node_repo::QueryPlanNodeRepository,
                operation::{BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation},
            },
            QueryPlanTree,
        },
        QueryPlan,
    };
    use crate::sql_processor::sql_processor_context::SqlProcessorContext;

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_query_executor() -> ApllodbResult<()> {
        fn engine() -> MockStorageEngine {
            let mut engine = default_mock_engine();
            engine.expect_with_tx().returning(|| {
                let mut with_tx = MockWithTxMethods::new();
                // mocking select()
                mock_select(&mut with_tx, &FULL_MODELS);
                with_tx
            });
            engine
        }

        #[derive(Clone)]
        struct TestRunner {
            context: Arc<SqlProcessorContext<MockStorageEngine>>,

            in_plan_tree: Option<QueryPlanTree>,
            expected_select_records: Option<Vec<Row>>,
        }
        impl TestRunner {
            fn new() -> Self {
                Self {
                    context: Arc::new(SqlProcessorContext::new(engine())),
                    in_plan_tree: None,
                    expected_select_records: None,
                }
            }

            fn add_query_plan_root(
                self,
                f: impl FnOnce(&QueryPlanNodeRepository) -> QueryPlanNodeId,
            ) -> Self {
                let mut new_self = Self { ..self };

                let node_id = f(&new_self.context.node_repo);
                new_self.in_plan_tree = Some(QueryPlanTree::new(node_id));
                new_self
            }

            fn expect(self, expected_select_records: Vec<Row>) -> Self {
                let mut new_self = Self { ..self };

                new_self.expected_select_records = Some(expected_select_records);
                new_self
            }

            async fn run(self) -> ApllodbResult<()> {
                let in_plan_tree = self.in_plan_tree.unwrap();
                let expected_select_records = self.expected_select_records.unwrap();

                let query_plan = QueryPlan::new(in_plan_tree.clone());
                let result = QueryExecutor::run_directly(self.context.clone(), query_plan).await?;

                assert_eq!(result.collect::<Vec<Row>>(), expected_select_records);

                Ok(())
            }
        }

        // SeqScan (with storage engine layer projection)
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: RowProjectionQuery::All,
                    },
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1.clone(),
                PEOPLE_RECORD2.clone(),
                PEOPLE_RECORD3.clone(),
            ])
            .run()
            .await?;
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: RowProjectionQuery::ColumnIndexes(
                            RecordFieldRefSchema::factory(vec![People::ffr_id()]),
                        ),
                    },
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1
                    .clone()
                    .projection(&[People::field_pos(People::ffr_id())])?,
                PEOPLE_RECORD2
                    .clone()
                    .projection(&[People::field_pos(People::ffr_id())])?,
                PEOPLE_RECORD3
                    .clone()
                    .projection(&[People::field_pos(People::ffr_id())])?,
            ])
            .run()
            .await?;
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: RowProjectionQuery::ColumnIndexes(
                            RecordFieldRefSchema::factory(vec![People::ffr_age()]),
                        ),
                    },
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1
                    .clone()
                    .projection(&[People::field_pos(People::ffr_age())])?,
                PEOPLE_RECORD2
                    .clone()
                    .projection(&[People::field_pos(People::ffr_age())])?,
                PEOPLE_RECORD3
                    .clone()
                    .projection(&[People::field_pos(People::ffr_age())])?,
            ])
            .run()
            .await?;
        // Projection
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from(People::ffr_id())]
                            .into_iter()
                            .collect(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1
                    .clone()
                    .projection(&[People::field_pos(People::ffr_id())])?,
                PEOPLE_RECORD2
                    .clone()
                    .projection(&[People::field_pos(People::ffr_id())])?,
                PEOPLE_RECORD3
                    .clone()
                    .projection(&[People::field_pos(People::ffr_id())])?,
            ])
            .run()
            .await?;
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![FieldIndex::from(People::ffr_age())]
                            .into_iter()
                            .collect(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1
                    .clone()
                    .projection(&[People::field_pos(People::ffr_age())])?,
                PEOPLE_RECORD2
                    .clone()
                    .projection(&[People::field_pos(People::ffr_age())])?,
                PEOPLE_RECORD3
                    .clone()
                    .projection(&[People::field_pos(People::ffr_age())])?,
            ])
            .run()
            .await?;
        // HashJoin
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: People::schema().joined(&Body::schema()),
                        left_field: FieldIndex::from(People::ffr_id()),
                        right_field: FieldIndex::from(Body::ffr_people_id()),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Body::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1.clone().naive_join(BODY_RECORD1.clone()),
                PEOPLE_RECORD3.clone().naive_join(BODY_RECORD3.clone()),
            ])
            .run()
            .await?;
        TestRunner::new()
            // right has 2 same join keys
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: People::schema().joined(&Pet::schema()),
                        left_field: FieldIndex::from(People::ffr_id()),
                        right_field: FieldIndex::from(Pet::ffr_people_id()),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                }))
            })
            .expect(vec![
                PEOPLE_RECORD1.clone().naive_join(PET_RECORD1.clone()),
                PEOPLE_RECORD3.clone().naive_join(PET_RECORD3_1.clone()),
                PEOPLE_RECORD3.clone().naive_join(PET_RECORD3_2.clone()),
            ])
            .run()
            .await?;
        TestRunner::new()
            // left has 2 same join keys
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: Pet::schema().joined(&People::schema()),
                        left_field: FieldIndex::from(Pet::ffr_people_id()),
                        right_field: FieldIndex::from(People::ffr_id()),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                }))
            })
            .expect(vec![
                PET_RECORD1.clone().naive_join(PEOPLE_RECORD1.clone()),
                PET_RECORD3_1.clone().naive_join(PEOPLE_RECORD3.clone()),
                PET_RECORD3_2.clone().naive_join(PEOPLE_RECORD3.clone()),
            ])
            .run()
            .await?;
        TestRunner::new()
            // Eq comparison with Integer & SmallInt
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: People::schema().joined(&Pet::schema()),
                        left_field: FieldIndex::from(People::ffr_age()),
                        right_field: FieldIndex::from(Pet::ffr_age()),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: RowProjectionQuery::All,
                        },
                    })),
                }))
            })
            .expect(vec![PEOPLE_RECORD1.clone().naive_join(PET_RECORD1.clone())])
            .run()
            .await?;

        Ok(())
    }
}
