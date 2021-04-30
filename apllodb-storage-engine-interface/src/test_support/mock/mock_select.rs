use crate::{
    table::table_name::TableName,
    test_support::test_models::{Body, ModelsMock, People, Pet},
    test_support::MockWithTxMethods,
    RowProjectionQuery, Rows,
};
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
                    rows: Rows::new(People::schema(), models.people),
                },
                MockRows {
                    table_name: Body::table_name(),
                    rows: Rows::new(Body::schema(), models.body),
                },
                MockRows {
                    table_name: Pet::table_name(),
                    rows: Rows::new(Pet::schema(), models.pet),
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
        .returning(move |session, table_name, projection, selection| {
            let models = models.clone();
            let datum = MockDatum::from(models);

            let table = datum
                .tables
                .iter()
                .find(|table| table.table_name == table_name)
                .unwrap_or_else(|| panic!("table `{:?}` is undefined in ModelsMock", table_name));

            let rows = table.rows.clone();

            let rows = rows.selection(&selection);

            let rows = match projection {
                RowProjectionQuery::All => rows,
                RowProjectionQuery::ColumnIndexes(indexes) => rows.projection(&indexes).unwrap(),
            };

            async move { Ok((rows, session)) }.boxed_local()
        });
}
