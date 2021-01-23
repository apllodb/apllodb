pub(crate) mod mock_select_with_models;

use std::collections::HashSet;

use crate::{
    test_support::{MockStorageEngine, MockWithTxMethods},
    ProjectionQuery,
};
use apllodb_shared_components::{ColumnReference, FieldIndex, Record, RecordIterator, TableName};
use futures::FutureExt;

#[derive(Clone, PartialEq, Debug)]
struct MockTxDbDatum {
    tables: Vec<MockTxTableDatum>,
}

#[derive(Clone, PartialEq, Debug)]
struct MockTxTableDatum {
    table_name: TableName,
    records: Vec<Record>,
}

fn mock_select(engine: &mut MockStorageEngine, data: MockTxDbDatum) {
    let mut with_tx = MockWithTxMethods::new();
    with_tx
        .expect_select()
        .returning(move |session, table_name, projection| {
            let table = data
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
    engine.expect_with_tx().return_once(move || with_tx);
}
