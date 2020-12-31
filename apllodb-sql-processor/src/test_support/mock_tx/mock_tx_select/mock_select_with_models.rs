use crate::test_support::{
    mock_tx::MockTx,
    test_models::{Body, People, Pet},
};

use apllodb_shared_components::Record;

use super::{mock_select, MockTxDbDatum, MockTxTableDatum};

#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) struct ModelsMock {
    pub(crate) people: Vec<Record>,
    pub(crate) body: Vec<Record>,
    pub(crate) pet: Vec<Record>,
}

pub(crate) fn mock_select_with_models(tx: &mut MockTx, models: ModelsMock) {
    mock_select(
        tx,
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
