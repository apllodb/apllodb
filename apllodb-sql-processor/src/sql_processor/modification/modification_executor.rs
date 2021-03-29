use std::sync::Arc;

use apllodb_shared_components::{ApllodbSessionResult, SessionWithTx};
use apllodb_storage_engine_interface::{StorageEngine, WithTxMethods};

use crate::sql_processor::{
    query::{
        query_executor::QueryExecutor,
        query_plan::{query_plan_tree::QueryPlanTree, QueryPlan},
    },
    sql_processor_context::SqlProcessorContext,
};

use super::modification_plan::{
    modification_plan_tree::modification_plan_node::ModificationPlanNode, ModificationPlan,
};

/// Modification (INSERT, UPDATE, and DELETE) executor which inputs a ModificationPlan requests to storage engine.
#[derive(Clone, Debug, new)]
pub(crate) struct ModificationExecutor<Engine: StorageEngine> {
    context: Arc<SqlProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> ModificationExecutor<Engine> {
    pub(crate) async fn run(
        &self,
        session: SessionWithTx,
        plan: ModificationPlan,
    ) -> ApllodbSessionResult<SessionWithTx> {
        let query_executor = QueryExecutor::new(self.context.clone());
        let plan_tree = plan.plan_tree;
        match plan_tree.root {
            ModificationPlanNode::Insert(insert_node) => {
                let input_query_plan_root_id = insert_node.child;
                let (input, session) = query_executor
                    .run(
                        session,
                        QueryPlan::new(QueryPlanTree::new(input_query_plan_root_id)),
                    )
                    .await?;

                let session = self
                    .context
                    .engine
                    .with_tx()
                    .insert(
                        session,
                        insert_node.table_name,
                        input
                            .as_full_field_references()
                            .iter()
                            .map(|ffr| ffr.as_column_name())
                            .cloned()
                            .collect(),
                        input.into_sql_values(),
                    )
                    .await?;

                Ok(session)
            }
            ModificationPlanNode::Update(update_node) => {
                let session = self
                    .context
                    .engine
                    .with_tx()
                    .update(session, update_node.table_name, update_node.column_values)
                    .await?;

                Ok(session)
            }
        }
    }
}
