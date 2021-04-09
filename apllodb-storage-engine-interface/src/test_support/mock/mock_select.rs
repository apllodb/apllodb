use crate::{
    test_support::test_models::{Body, ModelsMock, People, Pet},
    test_support::MockWithTxMethods,
    RowProjectionQuery, Rows,
};
use apllodb_shared_components::TableName;
use futures::FutureExt;

#[derive(Clone, PartialEq, Debug)]
struct MockDatum {
    tables: Vec<MockRows>,
}

impl From<ModelsMock> for MockDatum {
    fn from(models: ModelsMock) -> Self {
        MockDatum {
            tables: vec![
                MockRows {
                    table_name: People::table_name(),
                    rows: Records::factory(People::schema(), models.people),
                },
                MockRows {
                    table_name: Body::table_name(),
                    rows: Records::factory(Body::schema(), models.body),
                },
                MockRows {
                    table_name: Pet::table_name(),
                    rows: Records::factory(Pet::schema(), models.pet),
                },
            ],
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MockRows {
    table_name: TableName,
    rows: Rows,
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

            let rows = table.rows.clone();

            let rows = match projection {
                RowProjectionQuery::All => rows,
                RowProjectionQuery::Schema(schema) => {
                    let fields: Vec<FieldIndex> = schema
                        .as_full_field_references()
                        .iter()
                        .map(|ffr| FieldIndex::from(ffr.clone()))
                        .collect();

                    records.projection(&fields).unwrap()
                }
            };

            async move { Ok((rows, session)) }.boxed_local()
        });
}
