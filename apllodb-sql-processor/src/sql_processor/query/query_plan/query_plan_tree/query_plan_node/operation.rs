use apllodb_shared_components::{Expression, SchemaIndex};
use apllodb_storage_engine_interface::{RowProjectionQuery, TableName};
use serde::{Deserialize, Serialize};

use crate::{
    records::{record_schema::RecordSchema, Records},
    select::ordering::Ordering,
};

/// Leaf operations, which generates [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum LeafPlanOperation {
    Values {
        records: Records,
    },
    SeqScan {
        table_name: TableName,
        projection: RowProjectionQuery,
    },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Unary operations, which inputs [RecordIterator](apllodb-shared-components::RecordIterator) and outputs [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum UnaryPlanOperation {
    Projection {
        fields: Vec<SchemaIndex>,
    },
    Selection {
        /// Expression here must be evaluated as BOOLEAN (NULL is FALSE in BOOLEAN context).
        /// Otherwise [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch).
        condition: Expression,
    },
    Sort {
        field_orderings: Vec<(SchemaIndex, Ordering)>,
    }, // TODO extend.
       // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}

/// Binary operations, which inputs two [RecordIterator](apllodb-shared-components::RecordIterator) and outputs one [RecordIterator](apllodb-shared-components::RecordIterator).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(crate) enum BinaryPlanOperation {
    HashJoin {
        joined_schema: RecordSchema,
        left_field: SchemaIndex,
        right_field: SchemaIndex,
    },
    // TODO extend.
    // See PostgreSQL's plan nodes: <https://github.com/postgres/postgres/blob/master/src/include/nodes/nodes.h#L42-L95>
}
