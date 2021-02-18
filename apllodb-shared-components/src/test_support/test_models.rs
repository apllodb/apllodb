use crate::{
    FromItem, FullFieldReference, NNSqlValue, Record, RecordFieldRefSchema, SqlValue, TableName,
    UnresolvedFieldReference,
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

    pub fn ufr_id() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "id")
    }
    pub fn ufr_age() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "age")
    }

    pub fn ffr_id() -> FullFieldReference {
        Self::ufr_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_age() -> FullFieldReference {
        Self::ufr_age()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(vec![Self::ffr_id(), Self::ffr_age()])
    }

    pub fn record(id: i64, age: i32) -> Record {
        Record::factory(vec![
            (Self::ffr_id(), SqlValue::NotNull(NNSqlValue::BigInt(id))),
            (Self::ffr_age(), SqlValue::NotNull(NNSqlValue::Integer(age))),
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

    pub fn ufr_id() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "id")
    }
    pub fn ufr_people_id() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "people_id")
    }
    pub fn ufr_height() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "height")
    }

    pub fn ffr_id() -> FullFieldReference {
        Self::ufr_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_people_id() -> FullFieldReference {
        Self::ufr_people_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_height() -> FullFieldReference {
        Self::ufr_height()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
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
            (Self::ffr_id(), SqlValue::NotNull(NNSqlValue::BigInt(id))),
            (
                Self::ffr_people_id(),
                SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
            ),
            (
                Self::ffr_height(),
                SqlValue::NotNull(NNSqlValue::Integer(height)),
            ),
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

    pub fn ufr_id() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "id")
    }
    pub fn ufr_people_id() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "people_id")
    }
    pub fn ufr_kind() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "kind")
    }
    pub fn ufr_age() -> UnresolvedFieldReference {
        UnresolvedFieldReference::factory_corr_cn(Self::table_name().as_str(), "age")
    }

    pub fn ffr_id() -> FullFieldReference {
        Self::ufr_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_people_id() -> FullFieldReference {
        Self::ufr_people_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_kind() -> FullFieldReference {
        Self::ufr_kind()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_age() -> FullFieldReference {
        Self::ufr_age()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
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
            (Self::ffr_id(), SqlValue::NotNull(NNSqlValue::BigInt(id))),
            (
                Self::ffr_people_id(),
                SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
            ),
            (
                Self::ffr_kind(),
                SqlValue::NotNull(NNSqlValue::Text(kind.to_string())),
            ),
            (
                Self::ffr_age(),
                SqlValue::NotNull(NNSqlValue::SmallInt(age)),
            ),
        ])
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ModelsMock {
    pub people: Vec<Record>,
    pub body: Vec<Record>,
    pub pet: Vec<Record>,
}
