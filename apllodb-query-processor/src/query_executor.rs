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
        let record_iter = match root {
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
                todo!()
            }
            PlanNode::Binary { op, left, right } => {
                todo!()
            }
        };
        Ok(record_iter)
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{ApllodbResult, FieldIndex, TableName};
    use apllodb_storage_engine_interface::ProjectionQuery;

    use crate::{
        query_plan::{
            plan_tree::{
                plan_node::{LeafPlanOperation, PlanNode},
                PlanTree,
            },
            QueryPlan,
        },
        test_support::{setup, test_storage_engine::TestStorageEngine},
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

        let tx = TestStorageEngine::begin()?;
        let executor = QueryExecutor::<'_, TestStorageEngine>::new(&tx);

        let mut records = executor.run(query_plan)?;

        let id_field = FieldIndex::from("id");
        let age_field = FieldIndex::from("age");

        let r1 = records.next().unwrap();
        assert_eq!(r1.get::<i32>(id_field.clone())?, 1);
        assert_eq!(r1.get::<i32>(age_field.clone())?, 13);

        let r2 = records.next().unwrap();
        assert_eq!(r2.get::<i32>(id_field.clone())?, 2);
        assert_eq!(r2.get::<i32>(age_field.clone())?, 70);

        let r3 = records.next().unwrap();
        assert_eq!(r3.get::<i32>(id_field.clone())?, 3);
        assert_eq!(r3.get::<i32>(age_field.clone())?, 35);

        assert!(records.next().is_none());

        Ok(())
    }
}
