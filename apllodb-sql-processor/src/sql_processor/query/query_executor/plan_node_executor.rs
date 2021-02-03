use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionResult, Expression, FieldIndex, Record, RecordIterator,
    SessionWithTx, SqlValueHashKey, TableName,
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
    ) -> ApllodbSessionResult<(RecordIterator, SessionWithTx)> {
        match op_leaf {
            LeafPlanOperation::DirectInput { records } => Ok((records, session)),
            LeafPlanOperation::SeqScan {
                table_name,
                projection,
            } => self.seq_scan(session, table_name, projection).await,
        }
    }

    pub(super) fn run_unary(
        &self,
        op_unary: UnaryPlanOperation,
        input_left: RecordIterator,
    ) -> ApllodbResult<RecordIterator> {
        match op_unary {
            UnaryPlanOperation::Projection { fields } => self.projection(input_left, fields),
            UnaryPlanOperation::Selection { condition: _ } => {
                todo!()
            }
        }
    }

    pub(super) fn run_binary(
        &self,
        op_binary: BinaryPlanOperation,
        input_left: RecordIterator,
        input_right: RecordIterator,
    ) -> ApllodbResult<RecordIterator> {
        match op_binary {
            // TODO type cast
            BinaryPlanOperation::HashJoin {
                left_field,
                right_field,
            } => self.hash_join(input_left, input_right, &left_field, &right_field),
        }
    }

    async fn seq_scan(
        &self,
        session: SessionWithTx,
        table_name: TableName,
        projection: ProjectionQuery,
    ) -> ApllodbSessionResult<(RecordIterator, SessionWithTx)> {
        self.engine
            .with_tx()
            .select(session, table_name, projection)
            .await
    }

    /// # Failures
    ///
    /// Failures from [Record::projection()](apllodb_shared_components::Record::projection).
    fn projection(
        &self,
        input_left: RecordIterator,
        fields: HashSet<FieldIndex>,
    ) -> ApllodbResult<RecordIterator> {
        let it = RecordIterator::new(
            input_left
                .map(|record| record.projection(&fields))
                .collect::<ApllodbResult<Vec<Record>>>()?,
        );
        Ok(it)
    }

    fn _selection(
        &self,
        input_left: RecordIterator,
        condition: Expression,
    ) -> ApllodbResult<RecordIterator> {
        let it = RecordIterator::new(
            input_left
                .filter_map(|record| match record.selection(&condition) {
                    Err(e) => Some(Err(e)),
                    Ok(b) => {
                        if b {
                            Some(Ok(record))
                        } else {
                            None
                        }
                    }
                })
                .collect::<ApllodbResult<Vec<Record>>>()?,
        );
        Ok(it)
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
        input_left: RecordIterator,
        input_right: RecordIterator,
        left_field: &FieldIndex,
        right_field: &FieldIndex,
    ) -> ApllodbResult<RecordIterator> {
        // TODO Create hash table from smaller input.
        let mut hash_table = HashMap::<SqlValueHashKey, Vec<Record>>::new();

        for left_record in input_left {
            let left_sql_value = left_record.get_sql_value(&left_field)?;
            hash_table
                .entry(SqlValueHashKey::from(left_sql_value))
                // FIXME Clone less. If join keys are unique, no need for clone.
                .and_modify(|records| records.push(left_record.clone()))
                .or_insert_with(|| vec![left_record]);
        }

        let mut ret = Vec::<Record>::new();

        for right_record in input_right {
            let right_sql_value = right_record.get_sql_value(&right_field)?;
            if let Some(left_records) = hash_table.get(&SqlValueHashKey::from(right_sql_value)) {
                ret.append(
                    &mut left_records
                        .iter()
                        .map(|left_record| left_record.clone().join(right_record.clone()))
                        .collect::<ApllodbResult<Vec<Record>>>()?,
                );
            }
        }
        Ok(RecordIterator::new(ret))
    }
}
