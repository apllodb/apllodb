use std::collections::HashSet;

use apllodb_shared_components::{FieldIndex, TableName};
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

/// Node of query plan tree.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum PlanNode {
    Leaf(PlanNodeLeaf),
    Unary(PlanNodeUnary),
    Binary(PlanNodeBinary),
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct PlanNodeLeaf {
    pub(crate) op: LeafPlanOperation,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct PlanNodeUnary {
    pub(crate) op: UnaryPlanOperation,
    pub(crate) left: Box<PlanNode>,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct PlanNodeBinary {
    pub(crate) op: BinaryPlanOperation,
    pub(crate) left: Box<PlanNode>,
    pub(crate) right: Box<PlanNode>,
}

/// Leaf operations, which generates [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum LeafPlanOperation {
    SeqScan {
        table_name: TableName,
        projection: ProjectionQuery,
    },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Unary operations, which inputs [RecordIterator](apllodb-shared-components::RecordIterator) and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum UnaryPlanOperation {
    Projection { fields: HashSet<FieldIndex> },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Binary operations, which inputs two [RecordIterator](apllodb-shared-components::RecordIterator) and outputs one [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum BinaryPlanOperation {
    HashJoin {
        left_field: FieldIndex,
        right_field: FieldIndex,
    },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}
