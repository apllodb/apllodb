use apllodb_shared_components::{Expression, FieldIndex, Record, TableName};
use apllodb_storage_engine_interface::{AliasDef, ProjectionQuery};
use serde::{Deserialize, Serialize};

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

/// Leaf operations, which generates [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum LeafPlanOperation {
    Values {
        records: Vec<Record>,
    },
    SeqScan {
        table_name: TableName,
        projection: ProjectionQuery,
        alias_def: AliasDef,
    },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Unary operations, which inputs [RecordIterator](apllodb-shared-components::RecordIterator) and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum UnaryPlanOperation {
    Projection {
        fields: Vec<FieldIndex>,
    },
    Selection {
        /// Expression here must be evaluated as BOOLEAN (NULL is FALSE in BOOLEAN context).
        /// Otherwise [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch).
        condition: Expression,
    },
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
