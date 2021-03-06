use std::sync::Arc;

use apllodb_shared_components::{
    test_support::{
        fixture::*,
        test_models::{Body, People, Pet},
    },
    Record, RecordFieldRefSchema,
};
use once_cell::sync::Lazy;

use crate::Rec;

fn record_to_rec(schema: RecordFieldRefSchema, record: Record) -> Rec {
    Rec::new(Arc::new(schema), record)
}

pub static PEOPLE_REC1: Lazy<Rec> =
    Lazy::new(|| record_to_rec(People::schema(), PEOPLE_RECORD1.clone()));
pub static PEOPLE_REC2: Lazy<Rec> =
    Lazy::new(|| record_to_rec(People::schema(), PEOPLE_RECORD2.clone()));
pub static PEOPLE_REC3: Lazy<Rec> =
    Lazy::new(|| record_to_rec(People::schema(), PEOPLE_RECORD3.clone()));

pub static BODY_REC1: Lazy<Rec> = Lazy::new(|| record_to_rec(Body::schema(), BODY_RECORD1.clone()));
pub static BODY_REC3: Lazy<Rec> = Lazy::new(|| record_to_rec(Body::schema(), BODY_RECORD3.clone()));

pub static PET_REC1: Lazy<Rec> = Lazy::new(|| record_to_rec(Pet::schema(), PET_RECORD1.clone()));
pub static PET_REC3_1: Lazy<Rec> =
    Lazy::new(|| record_to_rec(Pet::schema(), PET_RECORD3_1.clone()));
pub static PET_REC3_2: Lazy<Rec> =
    Lazy::new(|| record_to_rec(Pet::schema(), PET_RECORD3_2.clone()));
