use super::{
    node_id::QueryPlanNodeId,
    operation::{BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation},
};

#[derive(Clone, Debug)]
pub(crate) enum QueryPlanNodeKind {
    Leaf(QueryPlanNodeLeaf),
    Unary(QueryPlanNodeUnary),

    #[allow(dead_code)]
    Binary(QueryPlanNodeBinary),
}

#[derive(Clone, Debug)]
pub(crate) struct QueryPlanNodeLeaf {
    pub(crate) op: LeafPlanOperation,
}

#[derive(Clone, Debug)]
pub(crate) struct QueryPlanNodeUnary {
    pub(crate) op: UnaryPlanOperation,
    pub(crate) left: QueryPlanNodeId,
}

#[derive(Clone, Debug)]
pub(crate) struct QueryPlanNodeBinary {
    pub(crate) op: BinaryPlanOperation,
    pub(crate) left: QueryPlanNodeId,
    pub(crate) right: QueryPlanNodeId,
}
