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

        let node = self.context.node_repo.remove(node_id);
        match node.kind {
            QueryPlanNodeKind::Leaf(node_leaf) => executor.run_leaf(session, node_leaf.op).await,
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
        }
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use apllodb_shared_components::ApllodbResult;
    use apllodb_storage_engine_interface::{
        test_support::{
            default_mock_engine,
            fixture::*,
            mock_select,
            test_models::{Body, ModelsMock, People, Pet},
            MockWithTxMethods,
        },
        MockStorageEngine, RowProjectionQuery,
    };
    use pretty_assertions::assert_eq;

    use super::QueryExecutor;
    use crate::sql_processor::sql_processor_context::SqlProcessorContext;
    use crate::{
        aliaser::Aliaser,
        records::{record::Record, record_schema::RecordSchema},
        sql_processor::query::query_plan::{
            query_plan_tree::{
                query_plan_node::{
                    node_id::QueryPlanNodeId,
                    node_kind::{
                        QueryPlanNodeBinary, QueryPlanNodeKind, QueryPlanNodeLeaf,
                        QueryPlanNodeUnary,
                    },
                    node_repo::QueryPlanNodeRepository,
                    operation::{BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation},
                },
                QueryPlanTree,
            },
            QueryPlan,
        },
        test_support::fixture::*,
    };

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_query_executor() -> ApllodbResult<()> {
        fn engine() -> MockStorageEngine {
            let mut engine = default_mock_engine();
            engine.expect_with_tx().returning(|| {
                let mut with_tx = MockWithTxMethods::new();
                // mocking select()
                mock_select(&mut with_tx, ModelsMock::fx_full());
                with_tx
            });
            engine
        }

        #[derive(Clone)]
        struct TestRunner {
            context: Arc<SqlProcessorContext<MockStorageEngine>>,

            in_plan_tree: Option<QueryPlanTree>,
            expected_select_records: Option<Vec<Record>>,
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

            fn expect(self, expected_select_records: Vec<Record>) -> Self {
                let mut new_self = Self { ..self };

                new_self.expected_select_records = Some(expected_select_records);
                new_self
            }

            async fn run(self) -> ApllodbResult<()> {
                let in_plan_tree = self.in_plan_tree.unwrap();
                let expected_select_records = self.expected_select_records.unwrap();

                let query_plan = QueryPlan::new(in_plan_tree.clone());
                let result = QueryExecutor::run_directly(self.context.clone(), query_plan).await?;

                assert_eq!(result.collect::<Vec<Record>>(), expected_select_records);

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
                        aliaser: Aliaser::default(),
                    },
                }))
            })
            .expect(vec![
                Record::fx_people1(),
                Record::fx_people2(),
                Record::fx_people3(),
            ])
            .run()
            .await?;
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: RowProjectionQuery::ColumnIndexes(
                            vec![People::tc_id().into()].into_iter().collect(),
                        ),
                        aliaser: Aliaser::default(),
                    },
                }))
            })
            .expect(vec![
                Record::fx_people1()
                    .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                Record::fx_people2()
                    .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                Record::fx_people3()
                    .projection(&vec![People::tc_id().into()].into_iter().collect())?,
            ])
            .run()
            .await?;
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: People::table_name(),
                        projection: RowProjectionQuery::ColumnIndexes(
                            vec![People::tc_age().into()].into_iter().collect(),
                        ),
                        aliaser: Aliaser::default(),
                    },
                }))
            })
            .expect(vec![
                Record::fx_people1()
                    .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                Record::fx_people2()
                    .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                Record::fx_people3()
                    .projection(&vec![People::tc_age().into()].into_iter().collect())?,
            ])
            .run()
            .await?;
        // Projection
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![People::tc_id().into()].into_iter().collect(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                }))
            })
            .expect(vec![
                Record::fx_people1()
                    .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                Record::fx_people2()
                    .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                Record::fx_people3()
                    .projection(&vec![People::tc_id().into()].into_iter().collect())?,
            ])
            .run()
            .await?;
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: UnaryPlanOperation::Projection {
                        fields: vec![People::tc_age().into()].into_iter().collect(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                }))
            })
            .expect(vec![
                Record::fx_people1()
                    .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                Record::fx_people2()
                    .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                Record::fx_people3()
                    .projection(&vec![People::tc_age().into()].into_iter().collect())?,
            ])
            .run()
            .await?;
        // HashJoin
        TestRunner::new()
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: RecordSchema::from_row_schema(
                            &People::schema(),
                            Aliaser::default(),
                        )
                        .joined(&RecordSchema::from_row_schema(
                            &Body::schema(),
                            Aliaser::default(),
                        )),
                        left_field: People::tc_id().into(),
                        right_field: Body::tc_people_id().into(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Body::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                }))
            })
            .expect(vec![
                Record::fx_people1().join(Record::fx_body1().clone())?,
                Record::fx_people3().join(Record::fx_body3().clone())?,
            ])
            .run()
            .await?;
        TestRunner::new()
            // right has 2 same join keys
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: RecordSchema::from_row_schema(
                            &People::schema(),
                            Aliaser::default(),
                        )
                        .joined(&RecordSchema::from_row_schema(
                            &Pet::schema(),
                            Aliaser::default(),
                        )),
                        left_field: People::tc_id().into(),
                        right_field: Pet::tc_people_id().into(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                }))
            })
            .expect(vec![
                Record::fx_people1().join(Record::fx_pet1().clone())?,
                Record::fx_people3().join(Record::fx_pet3_1().clone())?,
                Record::fx_people3().join(Record::fx_pet3_2().clone())?,
            ])
            .run()
            .await?;
        TestRunner::new()
            // left has 2 same join keys
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: RecordSchema::from_row_schema(
                            &Pet::schema(),
                            Aliaser::default(),
                        )
                        .joined(&RecordSchema::from_row_schema(
                            &People::schema(),
                            Aliaser::default(),
                        )),
                        left_field: Pet::tc_people_id().into(),
                        right_field: People::tc_id().into(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                }))
            })
            .expect(vec![
                Record::fx_pet1().clone().join(Record::fx_people1())?,
                Record::fx_pet3_1().clone().join(Record::fx_people3())?,
                Record::fx_pet3_2().clone().join(Record::fx_people3())?,
            ])
            .run()
            .await?;
        TestRunner::new()
            // Eq comparison with Integer & SmallInt
            .add_query_plan_root(|repo| {
                repo.create(QueryPlanNodeKind::Binary(QueryPlanNodeBinary {
                    op: BinaryPlanOperation::HashJoin {
                        joined_schema: RecordSchema::from_row_schema(
                            &People::schema(),
                            Aliaser::default(),
                        )
                        .joined(&RecordSchema::from_row_schema(
                            &Pet::schema(),
                            Aliaser::default(),
                        )),
                        left_field: People::tc_age().into(),
                        right_field: Pet::tc_age().into(),
                    },
                    left: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: People::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                    right: repo.create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                        op: LeafPlanOperation::SeqScan {
                            table_name: Pet::table_name(),
                            projection: RowProjectionQuery::All,
                            aliaser: Aliaser::default(),
                        },
                    })),
                }))
            })
            .expect(vec![Record::fx_people1().join(Record::fx_pet1().clone())?])
            .run()
            .await?;

        Ok(())
    }
}
