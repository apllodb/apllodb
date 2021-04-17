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

pub static PEOPLE_RECORD1: Lazy<Record> = Lazy::new(|| Record::new(*PEOPLE_SCHEMA, *PEOPLE_ROW1));
pub static PEOPLE_RECORD2: Lazy<Record> = Lazy::new(|| Record::new(*PEOPLE_SCHEMA, *PEOPLE_ROW2));
pub static PEOPLE_RECORD3: Lazy<Record> = Lazy::new(|| Record::new(*PEOPLE_SCHEMA, *PEOPLE_ROW3));

pub static BODY_RECORD1: Lazy<Record> = Lazy::new(|| Record::new(*BODY_SCHEMA, *BODY_ROW1));
pub static BODY_RECORD3: Lazy<Record> = Lazy::new(|| Record::new(*BODY_SCHEMA, *BODY_ROW3));

pub static PET_RECORD1: Lazy<Record> = Lazy::new(|| Record::new(*PET_SCHEMA, *PET_ROW1));
pub static PET_RECORD3_1: Lazy<Record> = Lazy::new(|| Record::new(*PET_SCHEMA, *PET_ROW3_1));
pub static PET_RECORD3_2: Lazy<Record> = Lazy::new(|| Record::new(*PET_SCHEMA, *PET_ROW3_2));
