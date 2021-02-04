use crate::{
    test_support::{
        test_models::{Body, People, Pet},
        MockWithTxMethods,
    },
    ProjectionQuery,
};
use apllodb_shared_components::{FieldIndex, Record, RecordIterator, TableName};
use futures::FutureExt;

#[derive(Clone, PartialEq, Debug)]
pub struct ModelsMock {
    pub people: Vec<Record>,
    pub body: Vec<Record>,
    pub pet: Vec<Record>,
}

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
                    records: models.people,
                },
                MockDatumInTable {
                    table_name: Body::table_name(),
                    records: models.body,
                },
                MockDatumInTable {
                    table_name: Pet::table_name(),
                    records: models.pet,
                },
            ],
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MockDatumInTable {
    table_name: TableName,
    records: RecordIterator,
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
                ProjectionQuery::ColumnNames(column_names) => {
                    let fields: Vec<FieldIndex> = column_names
                        .into_iter()
                        .map(|cn| {
                            FieldIndex::from(format!("{}.{}", table_name.as_str(), cn.as_str()))
                        })
                        .collect();

                    records.projection(&fields).unwrap()
                }
            };

            async move { Ok((records, session)) }.boxed_local()
        });
}
