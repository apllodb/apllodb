use std::collections::HashSet;

use crate::{
    test_support::{
        test_models::{Body, People, Pet},
        MockWithTxMethods,
    },
    ProjectionQuery,
};
use apllodb_shared_components::{ColumnReference, FieldIndex, Record, RecordIterator, TableName};
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
    records: Vec<Record>,
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
                .expect(&format!("table `{:?}` is undefined in MockTx", table_name));

            let records = table.records.clone();

            let records = match projection {
                ProjectionQuery::All => RecordIterator::new(records),
                ProjectionQuery::ColumnNames(column_names) => {
                    let fields: HashSet<FieldIndex> = column_names
                        .into_iter()
                        .map(|cn| {
                            FieldIndex::InColumnReference(ColumnReference::new(
                                table_name.clone(),
                                cn,
                            ))
                        })
                        .collect();

                    let projected_records: Vec<Record> = records
                        .into_iter()
                        .map(|record| record.projection(&fields).unwrap())
                        .collect();

                    RecordIterator::new(projected_records)
                }
            };

            async move { Ok((records, session)) }.boxed_local()
        });
}
