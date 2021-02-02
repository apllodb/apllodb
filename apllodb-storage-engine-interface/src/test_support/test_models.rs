use apllodb_shared_components::{
    ColumnName, ColumnReference, FieldIndex, NNSqlValue, Record, SqlValue, TableName,
};

use crate::record;

/// - people:
///   - id BIGINT NOT NULL, PRIMARY KEY
///   - age INTEGER NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub struct People;
impl People {
    pub fn table_name() -> TableName {
        TableName::new("people").unwrap()
    }

    pub fn colref_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("id").unwrap())
    }
    pub fn colref_age() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("age").unwrap())
    }

    pub fn record(id: i64, age: i32) -> Record {
        record! {
            FieldIndex::InColumnReference(Self::colref_id()) => SqlValue::NotNull(NNSqlValue::BigInt(id)),
            FieldIndex::InColumnReference(Self::colref_age()) => SqlValue::NotNull(NNSqlValue::Integer(age))
        }
    }
}

/// - body:
///   - id BIGINT NOT NULL, PRIMARY KEY
///   - people_id BIGINT NOT NULL
///   - height INTEGER NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub struct Body;
impl Body {
    pub fn table_name() -> TableName {
        TableName::new("body").unwrap()
    }

    pub fn colref_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("id").unwrap())
    }
    pub fn colref_people_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("people_id").unwrap())
    }
    pub fn colref_height() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("height").unwrap())
    }

    pub fn record(id: i64, people_id: i64, height: i32) -> Record {
        record! {
            FieldIndex::InColumnReference(Self::colref_id()) => SqlValue::NotNull(NNSqlValue::BigInt(id)),
            FieldIndex::InColumnReference(Self::colref_people_id()) => SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
            FieldIndex::InColumnReference(Self::colref_height()) => SqlValue::NotNull(NNSqlValue::Integer(height))
        }
    }
}

/// - pet:
///   - id BIGINT NOT NULL, PRIMARY KEY
///   - people_id BIGINT NOT NULL
///   - kind TEXT NOT NULL
///   - age SMALLINT NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub struct Pet;
impl Pet {
    pub fn table_name() -> TableName {
        TableName::new("pet").unwrap()
    }

    pub fn colref_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("id").unwrap())
    }
    pub fn colref_people_id() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("people_id").unwrap())
    }
    pub fn colref_kind() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("kind").unwrap())
    }
    pub fn colref_age() -> ColumnReference {
        ColumnReference::new(Self::table_name(), ColumnName::new("age").unwrap())
    }

    pub fn record(id: i64, people_id: i64, kind: &str, age: i16) -> Record {
        record! {
            FieldIndex::InColumnReference(Self::colref_id()) => SqlValue::NotNull(NNSqlValue::BigInt(id)),
            FieldIndex::InColumnReference(Self::colref_people_id()) => SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
            FieldIndex::InColumnReference(Self::colref_kind()) => SqlValue::NotNull(NNSqlValue::Text(kind.to_string())),
            FieldIndex::InColumnReference(Self::colref_age()) => SqlValue::NotNull(NNSqlValue::SmallInt(age))
        }
    }
}
