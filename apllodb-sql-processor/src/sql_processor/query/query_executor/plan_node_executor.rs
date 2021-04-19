use std::{collections::HashSet, sync::Arc};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionResult, Expression, SchemaIndex, SessionWithTx,
};
use apllodb_storage_engine_interface::{
    RowProjectionQuery, StorageEngine, TableName, WithTxMethods,
};

use crate::{
    aliaser::Aliaser,
    records::Records,
    select::ordering::Ordering,
    sql_processor::{
        query::query_plan::query_plan_tree::query_plan_node::operation::{
            BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation,
        },
        sql_processor_context::SqlProcessorContext,
    },
};

#[derive(Clone, Debug, new)]
pub(super) struct PlanNodeExecutor<Engine: StorageEngine> {
    context: Arc<SqlProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> PlanNodeExecutor<Engine> {
    pub(super) async fn run_leaf(
        &self,
        session: SessionWithTx,
        op_leaf: LeafPlanOperation,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        match op_leaf {
            LeafPlanOperation::Values { records } => Ok((records, session)),
            LeafPlanOperation::SeqScan {
                table_name,
                projection,
                aliaser,
            } => {
                self.seq_scan(session, table_name, projection, aliaser)
                    .await
            }
        }
    }

    pub(super) fn run_unary(
        &self,
        op_unary: UnaryPlanOperation,
        input_left: Records,
    ) -> ApllodbResult<Records> {
        match op_unary {
            UnaryPlanOperation::Projection { fields } => self.projection(input_left, &fields),
            UnaryPlanOperation::Selection { condition } => self.selection(input_left, condition),
            UnaryPlanOperation::Sort {
                index_orderings: field_orderings,
            } => Ok(self.sort(input_left, field_orderings)),
        }
    }

    pub(super) fn run_binary(
        &self,
        op_binary: BinaryPlanOperation,
        input_left: Records,
        input_right: Records,
    ) -> ApllodbResult<Records> {
        match op_binary {
            // TODO type cast
            BinaryPlanOperation::HashJoin {
                joined_schema,
                left_field,
                right_field,
            } => input_left.hash_join(
                Arc::new(joined_schema),
                input_right,
                &left_field,
                &right_field,
            ),
        }
    }

    async fn seq_scan(
        &self,
        session: SessionWithTx,
        table_name: TableName,
        projection: RowProjectionQuery,
        aliaser: Aliaser,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        let (rows, session) = self
            .context
            .engine
            .with_tx()
            .select(session, table_name, projection)
            .await?;

        let records = Records::from_rows(rows, aliaser);
        Ok((records, session))
    }

    /// # Failures
    ///
    /// Failures from [Record::projection()](apllodb_shared_components::Record::projection).
    fn projection(
        &self,
        input_left: Records,
        indexes: &HashSet<SchemaIndex>,
    ) -> ApllodbResult<Records> {
        input_left.projection(indexes)
    }

    fn selection(&self, input_left: Records, condition: Expression) -> ApllodbResult<Records> {
        input_left.selection(&condition)
    }

    fn sort(&self, input_left: Records, field_orderings: Vec<(SchemaIndex, Ordering)>) -> Records {
        input_left.sort(&field_orderings)
    }
}
