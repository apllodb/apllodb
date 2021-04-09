use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionResult, Expression, FieldIndex, Ordering, Row, RecordPos,
    Records, SessionWithTx, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine, WithTxMethods};

use crate::sql_processor::{
    query::query_plan::query_plan_tree::query_plan_node::operation::{
        BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation,
    },
    sql_processor_context::SqlProcessorContext,
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
            } => self.seq_scan(session, table_name, projection).await,
        }
    }

    pub(super) fn run_unary(
        &self,
        op_unary: UnaryPlanOperation,
        input_left: Records,
    ) -> ApllodbResult<Records> {
        match op_unary {
            UnaryPlanOperation::Projection { fields } => self.projection(input_left, fields),
            UnaryPlanOperation::Selection { condition } => self.selection(input_left, condition),
            UnaryPlanOperation::Sort { field_orderings } => self.sort(input_left, field_orderings),
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
            } => input_left.hash_join(joined_schema, input_right, &left_field, &right_field),
        }
    }

    async fn seq_scan(
        &self,
        session: SessionWithTx,
        table_name: TableName,
        projection: ProjectionQuery,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        self.context
            .engine
            .with_tx()
            .select(session, table_name, projection)
            .await
    }

    /// # Failures
    ///
    /// Failures from [Record::projection()](apllodb_shared_components::Record::projection).
    fn projection(&self, input_left: Records, fields: Vec<FieldIndex>) -> ApllodbResult<Records> {
        let positions: Vec<RecordPos> = fields
            .iter()
            .map(|f| input_left.as_schema().resolve_index(f))
            .collect::<ApllodbResult<_>>()?;

        let schema = input_left.as_schema().projection(&fields)?;

        let records = input_left
            .map(|record| record.projection(&positions))
            .collect::<ApllodbResult<Vec<Row>>>()?;

        Ok(Records::new(schema, records))
    }

    fn selection(&self, input_left: Records, condition: Expression) -> ApllodbResult<Records> {
        input_left.selection(&condition)
    }

    fn sort(
        &self,
        input_left: Records,
        field_orderings: Vec<(FieldIndex, Ordering)>,
    ) -> ApllodbResult<Records> {
        input_left.sort(&field_orderings)
    }
}
