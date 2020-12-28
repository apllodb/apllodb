use std::collections::HashMap;

use apllodb_shared_components::{ApllodbResult, Record, RecordIterator, SqlValue};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

use crate::query_plan::plan_tree::plan_node::{
    BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation,
};

#[derive(Debug, new)]
pub(super) struct PlanNodeExecutor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> PlanNodeExecutor<'exe, Engine> {
    pub(super) fn run_leaf(&self, op_leaf: LeafPlanOperation) -> ApllodbResult<RecordIterator> {
        let output = match op_leaf {
            LeafPlanOperation::SeqScan {
                table_name,
                projection,
            } => {
                let row_iter = self.tx.select(&table_name, projection)?;
                RecordIterator::new(row_iter)
            }
        };
        Ok(output)
    }

    pub(super) fn run_unary(
        &self,
        op_unary: UnaryPlanOperation,
        input_left: RecordIterator,
    ) -> ApllodbResult<RecordIterator> {
        let output = match op_unary {
            UnaryPlanOperation::Projection { fields } => RecordIterator::new(
                input_left
                    .map(|record| record.projection(&fields))
                    .collect::<ApllodbResult<Vec<Record>>>()?,
            ),
        };
        Ok(output)
    }

    pub(super) fn run_binary(
        &self,
        op_binary: BinaryPlanOperation,
        input_left: RecordIterator,
        input_right: RecordIterator,
    ) -> ApllodbResult<RecordIterator> {
        let output = match op_binary {
            // TODO type cast
            BinaryPlanOperation::HashJoin {
                left_field,
                right_field,
            } => {
                // TODO Create hash table from smaller input.
                let mut hash_table = HashMap::<SqlValue, Vec<Record>>::new();

                for left_record in input_left {
                    // FIXME Clone less. If join keys are unique, no need for clone.
                    let left_sql_value = left_record.get_sql_value(&left_field)?.clone();
                    hash_table
                        .entry(left_sql_value)
                        .and_modify(|records| records.push(left_record.clone()))
                        .or_insert_with(|| vec![left_record]);
                }

                let mut ret = Vec::<Record>::new();

                for right_record in input_right {
                    let right_sql_value = right_record.get_sql_value(&right_field)?.clone();
                    if let Some(left_records) = hash_table.get(&right_sql_value) {
                        ret.append(
                            &mut left_records
                                .iter()
                                .map(|left_record| left_record.clone().join(right_record.clone()))
                                .collect::<ApllodbResult<Vec<Record>>>()?,
                        );
                    }
                }
                RecordIterator::new(ret)
            }
        };
        Ok(output)
    }
}
