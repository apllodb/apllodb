use apllodb_shared_components::{NnSqlValue, RPos, SchemaIndex, SqlValue};

use crate::{
    rows::{row::Row, row_schema::RowSchema},
    table::table_name::TableName,
    table_column_name::TableColumnName,
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

    pub fn tc_id() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "id")
    }
    pub fn tc_age() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "age")
    }

    pub fn schema() -> RowSchema {
        RowSchema::factory(vec![Self::tc_id(), Self::tc_age()])
    }

    pub fn pos(afn: TableColumnName) -> RPos {
        let (pos, _) = Self::schema().index(&SchemaIndex::from(&afn)).unwrap();
        pos
    }

    pub fn row(id: i64, age: i32) -> Row {
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

    pub fn tc_id() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "id")
    }
    pub fn tc_people_id() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "people_id")
    }
    pub fn tc_height() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "height")
    }

    pub fn schema() -> RowSchema {
        RowSchema::factory(vec![Self::tc_id(), Self::tc_people_id(), Self::tc_height()])
    }

    pub fn row(id: i64, people_id: i64, height: i32) -> Row {
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

    pub fn tc_id() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "id")
    }
    pub fn tc_people_id() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "people_id")
    }
    pub fn tc_kind() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "kind")
    }
    pub fn tc_age() -> TableColumnName {
        TableColumnName::factory(Self::table_name().as_str(), "age")
    }

    pub fn schema() -> RowSchema {
        RowSchema::factory(vec![
            Self::tc_id(),
            Self::tc_people_id(),
            Self::tc_kind(),
            Self::tc_age(),
        ])
    }

    pub fn row(id: i64, people_id: i64, kind: &str, age: i16) -> Row {
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
