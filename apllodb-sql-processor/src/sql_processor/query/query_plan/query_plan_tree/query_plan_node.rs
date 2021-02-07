use apllodb_shared_components::{FieldIndex, SqlValues, TableName};
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

/// Node of query plan tree.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum QueryPlanNode {
    Leaf(QueryPlanNodeLeaf),
    Unary(QueryPlanNodeUnary),
    Binary(QueryPlanNodeBinary),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct QueryPlanNodeLeaf {
    pub(crate) op: LeafPlanOperation,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct QueryPlanNodeUnary {
    pub(crate) op: UnaryPlanOperation,
    pub(crate) left: Box<QueryPlanNode>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct QueryPlanNodeBinary {
    pub(crate) op: BinaryPlanOperation,
    pub(crate) left: Box<QueryPlanNode>,
    pub(crate) right: Box<QueryPlanNode>,
}

/// Leaf operations, which generates [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum LeafPlanOperation {
    Values {
        values_vec: Vec<SqlValues>,
    },
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
    Projection { fields: Vec<FieldIndex> },
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
