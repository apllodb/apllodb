pub(crate) mod operation;

use operation::{BinaryPlanOperation, LeafPlanOperation, UnaryPlanOperation};

/// Node of query plan tree.
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum QueryPlanNode {
    Leaf(QueryPlanNodeLeaf),
    Unary(QueryPlanNodeUnary),

    #[allow(dead_code)]
    Binary(QueryPlanNodeBinary),
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct QueryPlanNodeLeaf {
    pub(crate) op: LeafPlanOperation,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct QueryPlanNodeUnary {
    pub(crate) op: UnaryPlanOperation,
    pub(crate) left: Box<QueryPlanNode>,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct QueryPlanNodeBinary {
    pub(crate) op: BinaryPlanOperation,
    pub(crate) left: Box<QueryPlanNode>,
    pub(crate) right: Box<QueryPlanNode>,
}
