use crate::{test_support::MockWithTxMethods, ProjectionQuery};
use apllodb_shared_components::{
    test_support::test_models::{Body, ModelsMock, People, Pet},
    FieldIndex, Records, TableName,
};
use futures::FutureExt;

#[derive(Clone, PartialEq, Debug)]
struct MockDatum {
    tables: Vec<MockDatumInTable>,
}

impl From<ModelsMock> for MockDatum {
    fn from(models: ModelsMock) -> Self {
        MockDatum {
            tables: vec![
                MockDatumInTable {
                    table_name: People::table_name(),
                    records: Records::factory(People::schema(), models.people),
                },
                MockDatumInTable {
                    table_name: Body::table_name(),
                    records: Records::factory(Body::schema(), models.body),
                },
                MockDatumInTable {
                    table_name: Pet::table_name(),
                    records: Records::factory(Pet::schema(), models.pet),
                },
            ],
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MockDatumInTable {
    table_name: TableName,
    records: Records,
}

pub fn mock_select(with_tx: &mut MockWithTxMethods, models: &'static ModelsMock) {
    with_tx
        .expect_select()
        .returning(move |session, table_name, projection| {
            let models = models.clone();
            let datum = MockDatum::from(models);

            let table = datum
                .tables
                .iter()
                .find(|table| table.table_name == table_name)
                .unwrap_or_else(|| panic!("table `{:?}` is undefined in ModelsMock", table_name));

            let records = table.records.clone();

            let records = match projection {
                ProjectionQuery::All => records,
                ProjectionQuery::Schema(schema) => {
                    let fields: Vec<FieldIndex> = schema
                        .as_full_field_references()
                        .iter()
                        .map(|ffr| FieldIndex::from(ffr.clone()))
                        .collect();

                    records.projection(&fields).unwrap()
                }
            };

            async move { Ok((records, session)) }.boxed_local()
        });
}
