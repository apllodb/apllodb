use apllodb_shared_components::{ColumnName, ColumnReference, DataType, DataTypeKind, FieldIndex, Record, SqlValue, TableName};

use crate::record;

/// - people:
///   - id INTEGER NOT NULL
///   - age INTEGER NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct People;
impl People {
    pub(crate) fn table_name() -> TableName {
        TableName::new("people").unwrap()
    }

    pub(crate) fn colref_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("id").unwrap())
    }
    pub(crate) fn colref_age() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("age").unwrap())
    }

    pub(crate) fn record(id: i32, age: i32) -> Record {
        record! {
            FieldIndex::InColumnReference(Self::colref_id()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &id).unwrap(),
            FieldIndex::InColumnReference(Self::colref_age()) => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &age).unwrap()
        }
    }
}
