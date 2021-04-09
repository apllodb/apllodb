use once_cell::sync::Lazy;

use crate::rows::row::Row;

use super::test_models::{Body, ModelsMock, People, Pet};

pub static PEOPLE_ROW1: Lazy<Row> = Lazy::new(|| People::row(1, 13));
pub static PEOPLE_ROW2: Lazy<Row> = Lazy::new(|| People::row(2, 70));
pub static PEOPLE_ROW3: Lazy<Row> = Lazy::new(|| People::row(3, 35));

pub static BODY_ROW1: Lazy<Row> = Lazy::new(|| Body::row(1, 1, 145));
pub static BODY_ROW3: Lazy<Row> = Lazy::new(|| Body::row(2, 3, 175));

pub static PET_ROW1: Lazy<Row> = Lazy::new(|| Pet::row(1, 1, "dog", 13));
pub static PET_ROW3_1: Lazy<Row> = Lazy::new(|| Pet::row(2, 3, "dog", 5));
pub static PET_ROW3_2: Lazy<Row> = Lazy::new(|| Pet::row(3, 3, "cat", 3));

pub static FULL_MODELS: Lazy<ModelsMock> = Lazy::new(|| ModelsMock {
    people: vec![
        PEOPLE_ROW1.clone(),
        PEOPLE_ROW2.clone(),
        PEOPLE_ROW3.clone(),
    ],
    body: vec![BODY_ROW1.clone(), BODY_ROW3.clone()],
    pet: vec![PET_ROW1.clone(), PET_ROW3_1.clone(), PET_ROW3_2.clone()],
});

pub static PET_MODELS: Lazy<ModelsMock> = Lazy::new(|| ModelsMock {
    people: vec![],
    body: vec![],
    pet: vec![PET_ROW1.clone(), PET_ROW3_1.clone(), PET_ROW3_2.clone()],
});
