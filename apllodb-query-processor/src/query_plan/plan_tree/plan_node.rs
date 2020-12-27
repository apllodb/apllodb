use apllodb_shared_components::TableName;
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

/// Node of query plan tree.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum PlanNode {
    Leaf {
        op: LeafPlanOperation,
    },
    Unary {
        op: UnaryPlanOperation,
        left: Box<PlanNode>,
    },
    Binary {
        op: BinaryPlanOperation,
        left: Box<PlanNode>,
        right: Box<PlanNode>,
    },
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum UnaryPlanOperation {
    // TODO extend.
// See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Binary operations, which inputs two [RecordIterator](apllodb-shared-components::RecordIterator) and outputs one [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum BinaryPlanOperation {
    // TODO extend.
// See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}
