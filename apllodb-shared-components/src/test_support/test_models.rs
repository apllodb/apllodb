use crate::{
    data_structure::row::record_pos::RecordPos,
    record_index::named_record_index::NamedRecordIndex, AliasedFieldName, NnSqlValue, Row,
    RecordIndex, RecordSchema, SqlValue, TableName,
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

    pub fn afn_id() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "id")
    }
    pub fn afn_age() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "age")
    }

    pub fn schema() -> RecordSchema {
        RecordSchema::factory(vec![Self::afn_id(), Self::afn_age()])
    }

    pub fn field_pos(afn: AliasedFieldName) -> RecordPos {
        let (pos, _) = Self::schema().index(&NamedRecordIndex::from(&afn)).unwrap();
        pos
    }

    pub fn record(id: i64, age: i32) -> Row {
        Row::factory(vec![
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

    pub fn afn_id() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "id")
    }
    pub fn afn_people_id() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "people_id")
    }
    pub fn afn_height() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "height")
    }

    pub fn schema() -> RecordSchema {
        RecordSchema::factory(vec![
            Self::afn_id(),
            Self::afn_people_id(),
            Self::afn_height(),
        ])
    }

    pub fn record(id: i64, people_id: i64, height: i32) -> Row {
        Row::factory(vec![
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

    pub fn afn_id() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "id")
    }
    pub fn afn_people_id() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "people_id")
    }
    pub fn afn_kind() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "kind")
    }
    pub fn afn_age() -> AliasedFieldName {
        AliasedFieldName::factory(Self::table_name().as_str(), "age")
    }

    pub fn schema() -> RecordSchema {
        RecordSchema::factory(vec![
            Self::afn_id(),
            Self::afn_people_id(),
            Self::afn_kind(),
            Self::afn_age(),
        ])
    }

    pub fn record(id: i64, people_id: i64, kind: &str, age: i16) -> Row {
        Row::factory(vec![
            SqlValue::NotNull(NnSqlValue::BigInt(id)),
            SqlValue::NotNull(NnSqlValue::BigInt(people_id)),
            SqlValue::NotNull(NnSqlValue::Text(kind.to_string())),
            SqlValue::NotNull(NnSqlValue::SmallInt(age)),
        ])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ModelsMock {
    pub people: Vec<Row>,
    pub body: Vec<Row>,
    pub pet: Vec<Row>,
}
