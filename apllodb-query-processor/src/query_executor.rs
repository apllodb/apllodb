use apllodb_shared_components::{ApllodbResult, RecordIterator};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

use crate::query_plan::{
    plan_tree::plan_node::{LeafPlanOperation, PlanNode},
    QueryPlan,
};

/// Query executor which inputs a [QueryPlan](crate::query_plan::QueryPlan) and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(crate) struct QueryExecutor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> QueryExecutor<'exe, Engine> {
    pub(crate) fn run(&self, plan: QueryPlan) -> ApllodbResult<RecordIterator> {
        let plan_tree = plan.plan_tree;
        let root = plan_tree.root;
        self.run_dfs_post_order(root)
    }

    /// Runs `node` in post-order and returns `node`'s output.
    ///
    /// 1. Runs left child node and get output if exists.
    /// 2. Runs left child node and get output if exists.
    /// 3. Runs this `node` using inputs from left & right nodes if exist.
    /// 4. Returns `node`'s output.
    fn run_dfs_post_order(&self, node: PlanNode) -> ApllodbResult<RecordIterator> {
        let output = match node {
            PlanNode::Leaf { op } => match op {
                LeafPlanOperation::SeqScan {
                    table_name,
                    projection,
                } => {
                    let row_iter = self.tx.select(&table_name, projection)?;
                    RecordIterator::new(row_iter)
                }
            },
            PlanNode::Unary { op, left } => {
                let left_input = self.run_dfs_post_order(*left)?;
                todo!()
            }
            PlanNode::Binary { op, left, right } => {
                let left_input = self.run_dfs_post_order(*left)?;
                let right_input = self.run_dfs_post_order(*right)?;
                todo!()
            }
        };
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use apllodb_shared_components::{
        ApllodbResult, DataType, DataTypeKind, FieldIndex, Record, SqlValue, TableName,
    };
    use apllodb_storage_engine_interface::ProjectionQuery;

    use crate::{
        query_plan::{
            plan_tree::{
                plan_node::{LeafPlanOperation, PlanNode},
                PlanTree,
            },
            QueryPlan,
        },
        record,
        test_support::{
            setup,
            stub_storage_engine::{
                stub_data::StubTable, StubData, StubRowIterator, StubStorageEngine,
            },
        },
    };

    use super::QueryExecutor;

    #[test]
    fn test_seq_scan_only_plan() -> ApllodbResult<()> {
        setup();

        let plan_tree = PlanTree::new(PlanNode::Leaf {
            op: LeafPlanOperation::SeqScan {
                table_name: TableName::new("t")?,
                projection: ProjectionQuery::All,
            },
        });
        let query_plan = QueryPlan::new(plan_tree);

        let tname_t = TableName::new("t")?;

        let tx = StubStorageEngine::begin_stub_tx(StubData::new(vec![StubTable::new(
            tname_t.clone(),
            StubRowIterator::from(vec![
                record! {
                    "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
                    "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &13i32)?
                },
                record! {
                    "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &2i32)?,
                    "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &70i32)?
                },
                record! {
                    "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
                    "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &35i32)?
                },
            ]),
        )]))?;

        let executor = QueryExecutor::<'_, StubStorageEngine>::new(&tx);

        let expected = vec![
            record! {
                "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
                "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &13i32)?
            },
            record! {
                "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &2i32)?,
                "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &77i32)?
            },
            record! {
                "id" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &3i32)?,
                "age" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &35i32)?
            },
        ];

        let result = executor.run(query_plan)?;

        assert_eq!(result.collect::<Vec<Record>>(), expected);

        Ok(())
    }
}
