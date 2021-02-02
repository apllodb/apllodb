use apllodb_shared_components::Record;
use once_cell::sync::Lazy;

use super::{
    test_models::{Body, People, Pet},
    ModelsMock,
};

pub static T_PEOPLE_R1: Lazy<Record> = Lazy::new(|| People::record(1, 13));
pub static T_PEOPLE_R2: Lazy<Record> = Lazy::new(|| People::record(2, 70));
pub static T_PEOPLE_R3: Lazy<Record> = Lazy::new(|| People::record(3, 35));

pub static T_BODY_R1: Lazy<Record> = Lazy::new(|| Body::record(1, 1, 145));
pub static T_BODY_R3: Lazy<Record> = Lazy::new(|| Body::record(2, 3, 175));

pub static T_PET_R1: Lazy<Record> = Lazy::new(|| Pet::record(1, 1, "dog", 13));
pub static T_PET_R3_1: Lazy<Record> = Lazy::new(|| Pet::record(2, 3, "dog", 5));
pub static T_PET_R3_2: Lazy<Record> = Lazy::new(|| Pet::record(3, 3, "cat", 3));

pub static FULL_MODELS: Lazy<ModelsMock> = Lazy::new(|| ModelsMock {
    people: vec![
        T_PEOPLE_R1.clone(),
        T_PEOPLE_R2.clone(),
        T_PEOPLE_R3.clone(),
    ],
    body: vec![T_BODY_R1.clone(), T_BODY_R3.clone()],
    pet: vec![T_PET_R1.clone(), T_PET_R3_1.clone(), T_PET_R3_2.clone()],
});

pub static PET_MODELS: Lazy<ModelsMock> = Lazy::new(|| ModelsMock {
    people: vec![],
    body: vec![],
    pet: vec![T_PET_R1.clone(), T_PET_R3_1.clone(), T_PET_R3_2.clone()],
});
