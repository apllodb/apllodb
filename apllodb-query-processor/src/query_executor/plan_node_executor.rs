use apllodb_shared_components::{ApllodbResult, Record, RecordIterator};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

use crate::query_plan::plan_tree::plan_node::{LeafPlanOperation, UnaryPlanOperation};

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
                    .map(|mut record| {
                        record.projection(&fields)?;
                        Ok(record)
                    })
                    .collect::<ApllodbResult<Vec<Record>>>()?,
            ),
        };
        Ok(output)
    }
}
