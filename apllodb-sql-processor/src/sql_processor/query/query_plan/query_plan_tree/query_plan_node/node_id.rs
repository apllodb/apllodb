use serde::{Deserialize, Serialize};

/// Node ID of a query plan tree.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub(crate) struct QueryPlanNodeId(u32);

/// Monotonously increasing ID generator.
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct QueryPlanNodeIdGenerator(u32);
impl QueryPlanNodeIdGenerator {
    pub(crate) fn gen(&mut self) -> QueryPlanNodeId {
        self.0 += 1;
        QueryPlanNodeId(self.0)
    }
}
