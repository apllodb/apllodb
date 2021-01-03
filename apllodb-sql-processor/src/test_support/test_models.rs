use apllodb_shared_components::{
    ColumnName, ColumnReference, FieldIndex, Record, SqlType, SqlValue, TableName,
};

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
            FieldIndex::InColumnReference(Self::colref_id()) => SqlValue::pack(SqlType::integer(), &id).unwrap(),
            FieldIndex::InColumnReference(Self::colref_age()) => SqlValue::pack(SqlType::integer(), &age).unwrap()
        }
    }
}

/// - body:
///   - people_id INTEGER NOT NULL
///   - height INTEGER NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Body;
impl Body {
    pub(crate) fn table_name() -> TableName {
        TableName::new("body").unwrap()
    }

    pub(crate) fn colref_people_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("people_id").unwrap())
    }
    pub(crate) fn colref_height() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("height").unwrap())
    }

    pub(crate) fn record(people_id: i32, height: i32) -> Record {
        record! {
            FieldIndex::InColumnReference(Self::colref_people_id()) => SqlValue::pack(SqlType::integer(), &people_id).unwrap(),
            FieldIndex::InColumnReference(Self::colref_height()) => SqlValue::pack(SqlType::integer(), &height).unwrap()
        }
    }
}

/// - pet:
///   - people_id INTEGER NOT NULL
///   - kind TEXT NOT NULL
///   - age SMALLINT NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Pet;
impl Pet {
    pub(crate) fn table_name() -> TableName {
        TableName::new("pet").unwrap()
    }

    pub(crate) fn colref_people_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("people_id").unwrap())
    }
    pub(crate) fn colref_kind() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("kind").unwrap())
    }
    pub(crate) fn colref_age() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("age").unwrap())
    }

    pub(crate) fn record(people_id: i32, kind: &str, age: i16) -> Record {
        record! {
            FieldIndex::InColumnReference(Self::colref_people_id()) => SqlValue::pack(SqlType::integer(), &people_id).unwrap(),
            FieldIndex::InColumnReference(Self::colref_kind()) => SqlValue::pack(SqlType::text(), &kind.to_string()).unwrap(),
            FieldIndex::InColumnReference(Self::colref_age()) => SqlValue::pack(SqlType::small_int(), &age).unwrap()
        }
    }
}
