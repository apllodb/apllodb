use apllodb_shared_components::{
    Expression, FieldIndex, Ordering, RecordFieldRefSchema, Records, TableName,
};
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

/// Leaf operations, which generates [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum LeafPlanOperation {
    Values {
        records: Records,
    },
    SeqScan {
        table_name: TableName,
        projection: ProjectionQuery,
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
    Sort {
        field_orderings: Vec<(FieldIndex, Ordering)>,
    }, // TODO extend.
       // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Binary operations, which inputs two [RecordIterator](apllodb-shared-components::RecordIterator) and outputs one [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum BinaryPlanOperation {
    HashJoin {
        joined_schema: RecordFieldRefSchema,
        left_field: FieldIndex,
        right_field: FieldIndex,
    },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}
