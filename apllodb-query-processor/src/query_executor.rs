use apllodb_shared_components::{ApllodbResult, RecordIterator};
use serde::{Deserialize, Serialize};

use crate::query_plan::QueryPlan;

/// Query executor which inputs a [QueryPlan](crate::query_plan::QueryPlan) and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) struct QueryExecutor;

impl QueryExecutor {
    pub(crate) fn run(_plan: QueryPlan) -> ApllodbResult<RecordIterator> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::ApllodbResult;
    use apllodb_storage_engine_interface::ProjectionQuery;

    use crate::query_plan::{QueryPlan, plan_tree::{
        plan_node::{LeafPlanOperation, PlanNode},
        PlanTree,
    }};

    use super::QueryExecutor;

    #[test]
    fn test_seq_scan_only_plan() -> ApllodbResult<()> {
        let plan_tree = PlanTree::new(PlanNode::Leaf {
            op: LeafPlanOperation::SeqScan {
                projection: ProjectionQuery::All,
            },
        });
        let query_plan = QueryPlan::new(plan_tree);

        let records = QueryExecutor::run(query_plan)?;
        assert_eq!(records.count(), 3);

        Ok(())
    }
}
