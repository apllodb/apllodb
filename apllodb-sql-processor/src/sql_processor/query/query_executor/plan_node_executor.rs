use std::{collections::HashMap, rc::Rc};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionResult, Expression, FieldIndex, Ordering, Record,
    RecordFieldRefSchema, Records, SessionWithTx, SqlValueHashKey, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine, WithTxMethods};

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::{
    BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(super) struct PlanNodeExecutor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> PlanNodeExecutor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

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
            } => {
                let left_idx = input_left.as_schema().resolve_index(&left_field)?;
                let right_idx = input_right.as_schema().resolve_index(&right_field)?;
                self.hash_join(joined_schema, input_left, input_right, left_idx, right_idx)
            }
        }
    }

    async fn seq_scan(
        &self,
        session: SessionWithTx,
        table_name: TableName,
        projection: ProjectionQuery,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        self.engine
            .with_tx()
            .select(session, table_name, projection)
            .await
    }

    /// # Failures
    ///
    /// Failures from [Record::projection()](apllodb_shared_components::Record::projection).
    fn projection(&self, input_left: Records, fields: Vec<FieldIndex>) -> ApllodbResult<Records> {
        let idxs: Vec<usize> = fields
            .iter()
            .map(|f| input_left.as_schema().resolve_index(f))
            .collect::<ApllodbResult<_>>()?;

        let schema = input_left.as_schema().projection(&fields)?;

        let records = input_left
            .map(|record| record.projection(&idxs))
            .collect::<ApllodbResult<Vec<Record>>>()?;

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

    /// Join algorithm using hash table.
    /// It can be used with join keys' equality (like `ON t.id = s.t_id`).
    /// This algorithm's time-complexity is `max[O(len(input_left)), O(len(input_right))]` but uses relatively large memory.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in any record.
    fn hash_join(
        &self,
        joined_schema: RecordFieldRefSchema,
        input_left: Records,
        input_right: Records,
        left_idx: usize,
        right_idx: usize,
    ) -> ApllodbResult<Records> {
        // TODO Create hash table from smaller input.
        let mut hash_table = HashMap::<SqlValueHashKey, Vec<Record>>::new();

        for left_record in input_left {
            let left_sql_value = left_record.get_sql_value(left_idx)?;
            hash_table
                .entry(SqlValueHashKey::from(left_sql_value))
                // FIXME Clone less. If join keys are unique, no need for clone.
                .and_modify(|records| records.push(left_record.clone()))
                .or_insert_with(|| vec![left_record]);
        }

        let mut records = Vec::<Record>::new();
        for right_record in input_right {
            let right_sql_value = right_record.get_sql_value(right_idx)?;
            if let Some(left_records) = hash_table.get(&SqlValueHashKey::from(right_sql_value)) {
                records.append(
                    &mut left_records
                        .iter()
                        .map(|left_record| left_record.clone().join(right_record.clone()))
                        .collect::<Vec<Record>>(),
                );
            }
        }

        Ok(Records::new(joined_schema, records))
    }
}
