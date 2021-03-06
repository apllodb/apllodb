use crate::Record;
use once_cell::sync::Lazy;

use super::test_models::{Body, ModelsMock, People, Pet};

pub static PEOPLE_RECORD1: Lazy<Record> = Lazy::new(|| People::record(1, 13));
pub static PEOPLE_RECORD2: Lazy<Record> = Lazy::new(|| People::record(2, 70));
pub static PEOPLE_RECORD3: Lazy<Record> = Lazy::new(|| People::record(3, 35));

pub static BODY_RECORD1: Lazy<Record> = Lazy::new(|| Body::record(1, 1, 145));
pub static BODY_RECORD3: Lazy<Record> = Lazy::new(|| Body::record(2, 3, 175));

pub static PET_RECORD1: Lazy<Record> = Lazy::new(|| Pet::record(1, 1, "dog", 13));
pub static PET_RECORD3_1: Lazy<Record> = Lazy::new(|| Pet::record(2, 3, "dog", 5));
pub static PET_RECORD3_2: Lazy<Record> = Lazy::new(|| Pet::record(3, 3, "cat", 3));

pub static FULL_MODELS: Lazy<ModelsMock> = Lazy::new(|| ModelsMock {
    people: vec![
        PEOPLE_RECORD1.clone(),
        PEOPLE_RECORD2.clone(),
        PEOPLE_RECORD3.clone(),
    ],
    body: vec![BODY_RECORD1.clone(), BODY_RECORD3.clone()],
    pet: vec![
        PET_RECORD1.clone(),
        PET_RECORD3_1.clone(),
        PET_RECORD3_2.clone(),
    ],
});

pub static PET_MODELS: Lazy<ModelsMock> = Lazy::new(|| ModelsMock {
    people: vec![],
    body: vec![],
    pet: vec![
        PET_RECORD1.clone(),
        PET_RECORD3_1.clone(),
        PET_RECORD3_2.clone(),
    ],
});
