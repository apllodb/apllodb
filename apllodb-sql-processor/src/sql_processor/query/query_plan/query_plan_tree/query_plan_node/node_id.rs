use serde::{Deserialize, Serialize};

/// Node ID of a query plan tree.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub(crate) struct QueryPlanNodeId(u32);

#[derive(Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct QueryPlanNodeIdGenerator(u32);
impl QueryPlanNodeIdGenerator {
    pub(crate) fn gen(&self) -> QueryPlanNodeId {
        let next = self.0 + 1;
        QueryPlanNodeId(next)
    }
}
