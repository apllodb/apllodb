use crate::rows::row::Row;

use super::test_models::{Body, ModelsMock, People, Pet};

impl Row {
    pub fn fx_people1() -> Self {
        People::row(1, 13)
    }
    pub fn fx_people2() -> Self {
        People::row(2, 70)
    }
    pub fn fx_people3() -> Self {
        People::row(3, 35)
    }

    pub fn fx_body1() -> Self {
        Body::row(1, 1, 145)
    }
    pub fn fx_body3() -> Self {
        Body::row(2, 3, 175)
    }

    pub fn fx_pet1() -> Self {
        Pet::row(1, 1, "dog", 13)
    }
    pub fn fx_pet3_1() -> Self {
        Pet::row(2, 3, "dog", 5)
    }
    pub fn fx_pet3_2() -> Self {
        Pet::row(3, 3, "cat", 3)
    }
}

impl ModelsMock {
    pub fn fx_full() -> Self {
        Self {
            people: vec![Row::fx_people1(), Row::fx_people2(), Row::fx_people3()],
            body: vec![Row::fx_body1(), Row::fx_body3()],
            pet: vec![Row::fx_pet1(), Row::fx_pet3_1(), Row::fx_pet3_2()],
        }
    }

    pub fn fx_pet() -> Self {
        Self {
            people: vec![],
            body: vec![],
            pet: vec![Row::fx_pet1(), Row::fx_pet3_1(), Row::fx_pet3_2()],
        }
    }
}
