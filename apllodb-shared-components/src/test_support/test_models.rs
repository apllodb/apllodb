use crate::{
    FromItem, FullFieldReference, NNSqlValue, Record, RecordFieldRefSchema, SelectFieldReference,
    SqlValue, TableName,
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

    pub fn sfr_id() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "id")
    }
    pub fn sfr_age() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "age")
    }

    pub fn ffr_id() -> FullFieldReference {
        Self::sfr_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_age() -> FullFieldReference {
        Self::sfr_age()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(Self::table_name().as_str(), vec!["id", "age"])
    }

    pub fn record(id: i64, age: i32) -> Record {
        Record::factory(
            Self::table_name().as_str(),
            vec![
                ("id", SqlValue::NotNull(NNSqlValue::BigInt(id))),
                ("age", SqlValue::NotNull(NNSqlValue::Integer(age))),
            ],
        )
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

    pub fn sfr_id() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "id")
    }
    pub fn sfr_people_id() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "people_id")
    }
    pub fn sfr_height() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "height")
    }

    pub fn ffr_id() -> FullFieldReference {
        Self::sfr_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_people_id() -> FullFieldReference {
        Self::sfr_people_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_height() -> FullFieldReference {
        Self::sfr_height()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(
            Self::table_name().as_str(),
            vec!["id", "people_id", "height"],
        )
    }

    pub fn record(id: i64, people_id: i64, height: i32) -> Record {
        Record::factory(
            Self::table_name().as_str(),
            vec![
                ("id", SqlValue::NotNull(NNSqlValue::BigInt(id))),
                (
                    "people_id",
                    SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
                ),
                ("height", SqlValue::NotNull(NNSqlValue::Integer(height))),
            ],
        )
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

    pub fn sfr_id() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "id")
    }
    pub fn sfr_people_id() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "people_id")
    }
    pub fn sfr_kind() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "kind")
    }
    pub fn sfr_age() -> SelectFieldReference {
        SelectFieldReference::factory_corr_cn(Self::table_name().as_str(), "age")
    }

    pub fn ffr_id() -> FullFieldReference {
        Self::sfr_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_people_id() -> FullFieldReference {
        Self::sfr_people_id()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_kind() -> FullFieldReference {
        Self::sfr_kind()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }
    pub fn ffr_age() -> FullFieldReference {
        Self::sfr_age()
            .resolve(Some(FromItem::factory(Self::table_name().as_str())))
            .unwrap()
    }

    pub fn schema() -> RecordFieldRefSchema {
        RecordFieldRefSchema::factory(
            Self::table_name().as_str(),
            vec!["id", "people_id", "kind", "age"],
        )
    }

    pub fn record(id: i64, people_id: i64, kind: &str, age: i16) -> Record {
        Record::factory(
            Self::table_name().as_str(),
            vec![
                ("id", SqlValue::NotNull(NNSqlValue::BigInt(id))),
                (
                    "people_id",
                    SqlValue::NotNull(NNSqlValue::BigInt(people_id)),
                ),
                (
                    "kind",
                    SqlValue::NotNull(NNSqlValue::Text(kind.to_string())),
                ),
                ("age", SqlValue::NotNull(NNSqlValue::SmallInt(age))),
            ],
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ModelsMock {
    pub people: Vec<Record>,
    pub body: Vec<Record>,
    pub pet: Vec<Record>,
}
