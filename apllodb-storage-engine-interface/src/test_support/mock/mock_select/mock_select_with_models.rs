use crate::test_support::test_models::{Body, People, Pet};

use apllodb_shared_components::Record;

use super::{mock_select, MockTxDbDatum, MockTxTableDatum};
use crate::test_support::MockStorageEngine;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ModelsMock {
    pub people: Vec<Record>,
    pub body: Vec<Record>,
    pub pet: Vec<Record>,
}

pub fn mock_select_with_models(engine: &mut MockStorageEngine, models: ModelsMock) {
    mock_select(
        engine,
        MockTxDbDatum {
            tables: vec![
                MockTxTableDatum {
                    table_name: People::table_name(),
                    records: models.people,
                },
                MockTxTableDatum {
                    table_name: Body::table_name(),
                    records: models.body,
                },
                MockTxTableDatum {
                    table_name: Pet::table_name(),
                    records: models.pet,
                },
            ],
        },
    );
}
