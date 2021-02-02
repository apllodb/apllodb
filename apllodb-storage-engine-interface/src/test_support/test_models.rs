use apllodb_shared_components::{
    FieldIndex, FullFieldReference, NNSqlValue, Record, SqlValue, TableName,
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

    pub fn colref_id() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "id")
    }
    pub fn colref_age() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "age")
    }

    pub fn record(id: i64, age: i32) -> Record {
        record! {
            FieldIndex::InFullFieldReference(Self::colref_id()) => SqlValue::NotNull(NNSqlValue::BigInt(id)),
            FieldIndex::InFullFieldReference(Self::colref_age()) => SqlValue::NotNull(NNSqlValue::Integer(age))
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

    pub fn colref_id() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "id")
    }
    pub fn colref_people_id() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "people_id")
    }
    pub fn colref_height() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "height")
    }

    pub fn record(id: i64, people_id: i64, height: i32) -> Record {
        record! {
            FieldIndex::InFullFieldReference(Self::colref_id()) => SqlValue::NotNull(NNSqlValue::BigInt(id)),
            FieldIndex::InFullFieldReference(Self::colref_people_id()) => SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
            FieldIndex::InFullFieldReference(Self::colref_height()) => SqlValue::NotNull(NNSqlValue::Integer(height))
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

    pub fn colref_id() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "id")
    }
    pub fn colref_people_id() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "people_id")
    }
    pub fn colref_kind() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "kind")
    }
    pub fn colref_age() -> FullFieldReference {
        FullFieldReference::factory_table(Self::table_name().as_str(), "age")
    }

    pub fn record(id: i64, people_id: i64, kind: &str, age: i16) -> Record {
        record! {
            FieldIndex::InFullFieldReference(Self::colref_id()) => SqlValue::NotNull(NNSqlValue::BigInt(id)),
            FieldIndex::InFullFieldReference(Self::colref_people_id()) => SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
            FieldIndex::InFullFieldReference(Self::colref_kind()) => SqlValue::NotNull(NNSqlValue::Text(kind.to_string())),
            FieldIndex::InFullFieldReference(Self::colref_age()) => SqlValue::NotNull(NNSqlValue::SmallInt(age))
        }
    }
}
