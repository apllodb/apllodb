use std::sync::Arc;

use once_cell::sync::Lazy;

use apllodb_storage_engine_interface::test_support::{
    fixture::*,
    test_models::{Body, People, Pet},
};

use crate::{
    aliaser::Aliaser,
    records::{record::Record, record_schema::RecordSchema},
};

impl RecordSchema {
    fn fx_people() -> Self {
        Self::from_row_schema(&People::schema(), Aliaser::default())
    }

    fn fx_body() -> Self {
        Self::from_row_schema(&Body::schema(), Aliaser::default())
    }

    fn fx_people() -> Self {
        Self::from_row_schema(&People::schema(), Aliaser::default())
    }
}

static PEOPLE_SCHEMA: Lazy<Arc<RecordSchema>> = Lazy::new(|| {
    Arc::new(RecordSchema::from_row_schema(
        &People::schema(),
        Aliaser::default(),
    ))
});
static BODY_SCHEMA: Lazy<Arc<RecordSchema>> = Lazy::new(|| {
    Arc::new(RecordSchema::from_row_schema(
        &Body::schema(),
        Aliaser::default(),
    ))
});
static PET_SCHEMA: Lazy<Arc<RecordSchema>> = Lazy::new(|| {
    Arc::new(RecordSchema::from_row_schema(
        &Pet::schema(),
        Aliaser::default(),
    ))
});

pub static PEOPLE_RECORD1: Lazy<Record> =
    Lazy::new(|| Record::new(PEOPLE_SCHEMA.clone(), PEOPLE_ROW1.clone()));
pub static PEOPLE_RECORD2: Lazy<Record> =
    Lazy::new(|| Record::new(PEOPLE_SCHEMA.clone(), PEOPLE_ROW2.clone()));
pub static PEOPLE_RECORD3: Lazy<Record> =
    Lazy::new(|| Record::new(PEOPLE_SCHEMA.clone(), PEOPLE_ROW3.clone()));

pub static BODY_RECORD1: Lazy<Record> =
    Lazy::new(|| Record::new(BODY_SCHEMA.clone(), BODY_ROW1.clone()));
pub static BODY_RECORD3: Lazy<Record> =
    Lazy::new(|| Record::new(BODY_SCHEMA.clone(), BODY_ROW3.clone()));

pub static PET_RECORD1: Lazy<Record> =
    Lazy::new(|| Record::new(PET_SCHEMA.clone(), PET_ROW1.clone()));
pub static PET_RECORD3_1: Lazy<Record> =
    Lazy::new(|| Record::new(PET_SCHEMA.clone(), PET_ROW3_1.clone()));
pub static PET_RECORD3_2: Lazy<Record> =
    Lazy::new(|| Record::new(PET_SCHEMA.clone(), PET_ROW3_2.clone()));
