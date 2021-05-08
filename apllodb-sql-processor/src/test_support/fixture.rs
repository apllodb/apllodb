use std::sync::Arc;

use apllodb_storage_engine_interface::{
    test_support::test_models::{Body, People, Pet},
    Row,
};

use crate::{
    aliaser::Aliaser,
    records::{record::Record, record_schema::RecordSchema},
};

impl RecordSchema {
    pub fn fx_people() -> Self {
        Self::from_row_schema(&People::schema(), Aliaser::default())
    }

    pub fn fx_body() -> Self {
        Self::from_row_schema(&Body::schema(), Aliaser::default())
    }

    pub fn fx_pet() -> Self {
        Self::from_row_schema(&Pet::schema(), Aliaser::default())
    }
}

impl Record {
    pub fn fx_people1() -> Self {
        Self::new(Arc::new(RecordSchema::fx_people()), Row::fx_people1())
    }
    pub fn fx_people2() -> Self {
        Self::new(Arc::new(RecordSchema::fx_people()), Row::fx_people2())
    }
    pub fn fx_people3() -> Self {
        Self::new(Arc::new(RecordSchema::fx_people()), Row::fx_people3())
    }

    pub fn fx_body1() -> Self {
        Self::new(Arc::new(RecordSchema::fx_body()), Row::fx_body1())
    }
    pub fn fx_body3() -> Self {
        Self::new(Arc::new(RecordSchema::fx_body()), Row::fx_body3())
    }

    pub fn fx_pet1() -> Self {
        Self::new(Arc::new(RecordSchema::fx_pet()), Row::fx_pet1())
    }
    pub fn fx_pet3_1() -> Self {
        Self::new(Arc::new(RecordSchema::fx_pet()), Row::fx_pet3_1())
    }
    pub fn fx_pet3_2() -> Self {
        Self::new(Arc::new(RecordSchema::fx_pet()), Row::fx_pet3_2())
    }
}
