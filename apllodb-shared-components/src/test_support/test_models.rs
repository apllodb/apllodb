use crate::{
    FieldIndex, FullFieldReference, NnSqlValue, Record, RecordFieldRefSchema, SqlValue, TableName,
};

/// - people:
///   - id BIGINT NOT NULL, PRIMARY KEY
///   - age INTEGER NOT NULL
#[derive(Clone, PartialEq, Debug)]
pub struct People;
impl People {
    pub fn table_name() -> TableName {
        TableName::new("people").unwrap()
    }

    pub fn ffr_id() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "id")
    }
    pub fn ffr_age() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "age")
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(vec![Self::ffr_id(), Self::ffr_age()])
    }

    pub fn field_idx(ffr: FullFieldReference) -> usize {
        Self::schema()
            .resolve_index(&FieldIndex::from(ffr))
            .unwrap()
    }

    pub fn record(id: i64, age: i32) -> Record {
        Record::factory(vec![
            SqlValue::NotNull(NnSqlValue::BigInt(id)),
            SqlValue::NotNull(NnSqlValue::Integer(age)),
        ])
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

    pub fn ffr_id() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "id")
    }
    pub fn ffr_people_id() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "people_id")
    }
    pub fn ffr_height() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "height")
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(vec![
            Self::ffr_id(),
            Self::ffr_people_id(),
            Self::ffr_height(),
        ])
    }

    pub fn record(id: i64, people_id: i64, height: i32) -> Record {
        Record::factory(vec![
            SqlValue::NotNull(NnSqlValue::BigInt(id)),
            SqlValue::NotNull(NnSqlValue::BigInt(people_id)),
            SqlValue::NotNull(NnSqlValue::Integer(height)),
        ])
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

    pub fn ffr_id() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "id")
    }
    pub fn ffr_people_id() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "people_id")
    }
    pub fn ffr_kind() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "kind")
    }
    pub fn ffr_age() -> FullFieldReference {
        FullFieldReference::factory(Self::table_name().as_str(), "age")
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(vec![
            Self::ffr_id(),
            Self::ffr_people_id(),
            Self::ffr_kind(),
            Self::ffr_age(),
        ])
    }

    pub fn record(id: i64, people_id: i64, kind: &str, age: i16) -> Record {
        Record::factory(vec![
            SqlValue::NotNull(NnSqlValue::BigInt(id)),
            SqlValue::NotNull(NnSqlValue::BigInt(people_id)),
            SqlValue::NotNull(NnSqlValue::Text(kind.to_string())),
            SqlValue::NotNull(NnSqlValue::SmallInt(age)),
        ])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ModelsMock {
    pub people: Vec<Record>,
    pub body: Vec<Record>,
    pub pet: Vec<Record>,
}
